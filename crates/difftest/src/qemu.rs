use std::io::{BufReader, BufWriter, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

// Constants
// 32 general registers + 1 for PC + 32 for FPU registers
const DIFFTEST_REG_SIZE: usize = 32 + 1 + 32; // Adjust based on your architecture

// Error types
#[derive(Debug)]
pub enum GdbError {
    ConnectionFailed,
    InvalidResponse,
    ChecksumMismatch,
    IoError(std::io::Error),
}

impl From<std::io::Error> for GdbError {
    fn from(err: std::io::Error) -> Self {
        GdbError::IoError(err)
    }
}

// GDB connection structure
pub struct GdbConn {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    ack_mode: bool,
}

/// ref: https://www.embecosm.com/appnotes/ean4/embecosm-howto-rsp-server-ean4-issue-2.html
impl GdbConn {
    pub fn new(addr: &str, port: u16) -> Result<Self, GdbError> {
        let socket_addr: SocketAddr = format!("{}:{}", addr, port)
            .parse()
            .map_err(|_| GdbError::ConnectionFailed)?;

        let stream = TcpStream::connect(socket_addr).map_err(|_| GdbError::ConnectionFailed)?;

        // Enable TCP_NODELAY and SO_KEEPALIVE
        stream.set_nodelay(true)?;

        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream.try_clone()?);

        let mut conn = GdbConn {
            reader,
            writer,
            ack_mode: true,
        };

        // Send initial ACK
        conn.writer.write_all(b"+")?;
        conn.writer.flush()?;

        Ok(conn)
    }

    fn hex_encode(digit: u8) -> char {
        if digit > 9 {
            (b'a' + digit - 10) as char
        } else {
            (b'0' + digit) as char
        }
    }

    fn hex_nibble(hex: u8) -> u8 {
        if hex.is_ascii_digit() {
            hex - b'0'
        } else {
            hex.to_ascii_lowercase() - b'a' + 10
        }
    }

    fn decode_hex(msb: u8, lsb: u8) -> Option<u8> {
        if msb.is_ascii_hexdigit() && lsb.is_ascii_hexdigit() {
            Some(16 * Self::hex_nibble(msb) + Self::hex_nibble(lsb))
        } else {
            None
        }
    }

    fn decode_hex_bytes(hex_data: &[u8]) -> Result<Vec<u8>, GdbError> {
        if hex_data.len() % 2 != 0 {
            return Err(GdbError::InvalidResponse);
        }

        let mut result = Vec::with_capacity(hex_data.len() / 2);
        let mut i = 0;

        while i + 1 < hex_data.len() {
            if let Some(byte) = Self::decode_hex(hex_data[i], hex_data[i + 1]) {
                result.push(byte);
                i += 2;
            } else {
                return Err(GdbError::InvalidResponse);
            }
        }

        Ok(result)
    }

    fn calculate_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
    }

    pub fn send(&mut self, command: &[u8]) -> Result<(), GdbError> {
        loop {
            let checksum = Self::calculate_checksum(command);

            // Send packet: $<data>#<checksum>
            self.writer.write_all(b"$")?;
            self.writer.write_all(command)?;
            self.writer
                .write_all(format!("#{:02X}", checksum).as_bytes())?;
            self.writer.flush()?;

            if !self.ack_mode {
                break;
            }

            // Wait for ACK/NACK
            let mut ack = [0u8; 1];
            self.reader.read_exact(&mut ack)?;

            if ack[0] == b'+' {
                break; // ACK received
            }
            // Continue loop for NACK (-)
        }

        Ok(())
    }

    pub fn recv(&mut self) -> Result<Vec<u8>, GdbError> {
        loop {
            let packet = self.recv_packet()?;

            if !self.ack_mode {
                return Ok(packet.0);
            }

            if packet.1 {
                // Checksum OK, send ACK
                self.writer.write_all(b"+")?;
                self.writer.flush()?;
                return Ok(packet.0);
            } else {
                // Checksum failed, send NACK
                self.writer.write_all(b"-")?;
                self.writer.flush()?;
                // Continue loop to retry
            }
        }
    }

    fn recv_packet(&mut self) -> Result<(Vec<u8>, bool), GdbError> {
        let mut reply = Vec::new();
        let mut sum = 0u8;
        let mut escape = false;

        // Find packet start
        loop {
            let mut c = [0u8; 1];
            self.reader.read_exact(&mut c)?;
            if c[0] == b'$' {
                break;
            }
        }

        // Read packet content
        loop {
            let mut c = [0u8; 1];
            self.reader.read_exact(&mut c)?;
            let c = c[0];

            sum = sum.wrapping_add(c);

            match c {
                b'$' => {
                    // New packet, start over
                    reply.clear();
                    sum = 0;
                    escape = false;
                    continue;
                }
                b'#' => {
                    // End of packet
                    sum = sum.wrapping_sub(c); // Remove '#' from checksum

                    let mut checksum_bytes = [0u8; 2];
                    self.reader.read_exact(&mut checksum_bytes)?;

                    let checksum_ok = if let Some(expected_sum) =
                        Self::decode_hex(checksum_bytes[0], checksum_bytes[1])
                    {
                        sum == expected_sum
                    } else {
                        false
                    };

                    return Ok((reply, checksum_ok));
                }
                b'}' => {
                    // Escape character
                    escape = true;
                    continue;
                }
                b'*' => {
                    // RLE - simplified implementation
                    if !reply.is_empty() {
                        let mut count_byte = [0u8; 1];
                        self.reader.read_exact(&mut count_byte)?;
                        let count = count_byte[0];

                        if count >= 29 && count <= 126 && count != b'$' && count != b'#' {
                            let repeat_count = (count - 29) as usize;
                            let last_byte = reply[reply.len() - 1];
                            reply.extend(std::iter::repeat(last_byte).take(repeat_count));
                            sum = sum.wrapping_add(count);
                            continue;
                        }
                    }
                }
                _ => {}
            }

            let mut byte = c;
            if escape {
                byte ^= 0x20;
                escape = false;
            }

            reply.push(byte);
        }
    }
}

// Difftest implementation
pub struct DiffTest {
    conn: GdbConn,
    qemu_process: Option<Child>,
}

impl DiffTest {
    pub fn new(port: u16) -> Result<Self, GdbError> {
        // Start QEMU process
        let qemu_process = Command::new("qemu-system-riscv64") // Adjust for your ISA
            .args(&[
                "-S",
                "-gdb",
                &format!("tcp::{}", port),
                "-nographic",
                "-serial",
                "none",
                "-monitor",
                "none",
            ])
            .spawn()
            .map_err(|_| GdbError::ConnectionFailed)?;

        // Wait a bit for QEMU to start
        thread::sleep(Duration::from_millis(100));

        // Connect to GDB server
        let mut attempts = 0;
        let conn = loop {
            match GdbConn::new("127.0.0.1", port) {
                Ok(conn) => break conn,
                Err(_) if attempts < 1000 => {
                    attempts += 1;
                    thread::sleep(Duration::from_micros(1));
                    continue;
                }
                Err(e) => return Err(e),
            }
        };

        println!("Connected to QEMU with tcp::{} successfully", port);

        Ok(DiffTest {
            conn,
            qemu_process: Some(qemu_process),
        })
    }

    pub fn memcpy_to_qemu(&mut self, dest: u32, src: &[u8]) -> Result<bool, GdbError> {
        const MTU: usize = 8;
        let mut offset = 0;
        let mut dest_addr = dest;

        while offset < src.len() {
            let chunk_size = std::cmp::min(MTU, src.len() - offset);
            let chunk = &src[offset..offset + chunk_size];

            if !self.memcpy_to_qemu_small(dest_addr, chunk)? {
                return Ok(false);
            }

            dest_addr += chunk_size as u32;
            offset += chunk_size;
        }

        Ok(true)
    }

    pub fn memcpy_from_qemu(&mut self, src: u32, len: usize) -> Result<Vec<u8>, GdbError> {
        const MTU: usize = 8;
        let mut result = Vec::with_capacity(len);
        let mut offset = 0;
        let mut src_addr = src;

        while offset < len {
            let chunk_size = std::cmp::min(MTU, len - offset);
            let mut chunk = self.memcpy_from_qemu_small(src_addr, chunk_size)?;
            result.append(&mut chunk);

            src_addr += chunk_size as u32;
            offset += chunk_size;
        }

        Ok(result)
    }

    fn memcpy_to_qemu_small(&mut self, dest: u32, src: &[u8]) -> Result<bool, GdbError> {
        let mut command = format!("M{:#x},{}:", dest, src.len()).into_bytes();

        for &byte in src {
            command.push(GdbConn::hex_encode(byte >> 4) as u8);
            command.push(GdbConn::hex_encode(byte & 0xf) as u8);
        }

        self.conn.send(&command)?;
        let reply = self.conn.recv()?;

        Ok(reply == b"OK")
    }

    fn memcpy_from_qemu_small(&mut self, src: u32, len: usize) -> Result<Vec<u8>, GdbError> {
        let command = format!("m{:#x},{}", src, len);

        self.conn.send(command.as_bytes())?;
        let reply = self.conn.recv()?;

        // Parse hex response into bytes using the new decode_hex_bytes function
        let result = GdbConn::decode_hex_bytes(&reply)?;

        if result.len() != len {
            return Err(GdbError::InvalidResponse);
        }

        Ok(result)
    }

    pub fn get_regs(&mut self, regs: &mut [u64]) -> Result<bool, GdbError> {
        self.conn.send(b"g")?;
        let reply = self.conn.recv()?;

        let mut i = 0;
        let mut pos = 0;
        const REG_SIZE: usize = 8; // Assuming 64-bit registers
        const REG_HEX_SIZE: usize = 2 * REG_SIZE; // Each register is represented by 16 hex characters

        while i < regs.len() && pos + REG_HEX_SIZE <= reply.len() {
            let hex_str = &reply[pos..pos + REG_HEX_SIZE];
            regs[i] = hex::decode(hex_str)
                .map_err(|_| GdbError::InvalidResponse)?
                .iter()
                .rev()
                .fold(0u64, |acc, &b| (acc << 8) | b as u64);
            i += 1;
            pos += REG_HEX_SIZE;
        }

        Ok(true)
    }

    pub fn set_regs(&mut self, regs: &[u64]) -> Result<bool, GdbError> {
        let mut command = vec![b'G'];

        for &reg in regs {
            let bytes = reg.to_le_bytes();
            for byte in bytes {
                command.push(GdbConn::hex_encode(byte >> 4) as u8);
                command.push(GdbConn::hex_encode(byte & 0xf) as u8);
            }
        }

        self.conn.send(&command)?;
        let reply = self.conn.recv()?;

        Ok(reply == b"OK")
    }

    pub fn continues(&mut self) -> Result<bool, GdbError> {
        self.conn.send(b"vCont;c:1")?;
        let _reply = self.conn.recv()?;
        Ok(true)
    }

    pub fn single_step(&mut self) -> Result<bool, GdbError> {
        self.conn.send(b"vCont;s:1")?;
        let _reply = self.conn.recv()?;
        Ok(true)
    }

    pub fn breakpoint(&mut self, addr: u64) -> Result<bool, GdbError> {
        // self.talk(format!("Z0,{:x}, 4", addr).as_str());
        self.conn
            .send(format!("Z0,{:x},4", addr).as_bytes())
            .unwrap();
        let reply = self.conn.recv()?;
        Ok(reply == b"OK")
    }

    pub fn rm_breakpoint(&mut self, addr: u64) -> Result<bool, GdbError> {
        self.conn
            .send(format!("z0,{:x},4", addr).as_bytes())
            .unwrap();
        let reply = self.conn.recv()?;
        Ok(reply == b"OK")
    }

    // Difftest interface functions
    pub fn difftest_memcpy_to(&mut self, addr: u32, buf: &[u8]) -> Result<(), GdbError> {
        let ok = self.memcpy_to_qemu(addr, buf)?;
        assert!(ok);
        Ok(())
    }

    pub fn difftest_memcpy_from(&mut self, addr: u32, len: usize) -> Result<Vec<u8>, GdbError> {
        let data = self.memcpy_from_qemu(addr, len)?;
        Ok(data)
    }

    pub fn difftest_write_general_regs(&mut self, dut_regs: &[u64]) -> Result<(), GdbError> {
        assert_eq!(dut_regs.len(), DIFFTEST_REG_SIZE);
        let reg_count = DIFFTEST_REG_SIZE; // Assuming 32-bit registers
        let mut qemu_regs = vec![0; reg_count];

        // self.get_regs(&mut qemu_regs)?;
        qemu_regs.copy_from_slice(&dut_regs[..reg_count]);
        self.set_regs(&qemu_regs)?;

        Ok(())
    }

    pub fn difftest_read_general_regs(&mut self, dut_regs: &mut [u64]) -> Result<(), GdbError> {
        let reg_count = DIFFTEST_REG_SIZE;
        let mut qemu_regs = vec![0; reg_count];
        self.get_regs(&mut qemu_regs)?;
        dut_regs[..reg_count].copy_from_slice(&qemu_regs);

        Ok(())
    }

    pub fn difftest_exec(&mut self, n: u64) -> Result<(), GdbError> {
        for _ in 0..n {
            self.single_step()?;
        }
        Ok(())
    }

    pub fn difftest_read_csr_reg(&mut self, reg: usize) -> Result<u64, GdbError> {
        let command = format!("p{:x}", reg);
        println!("cmd: {}", command);
        self.conn.send(command.as_bytes())?;
        let reply = self.conn.recv()?;
        if reply.len() == 8 * 2 {
            let res = hex::decode(&reply)
                .map_err(|_| GdbError::InvalidResponse)?
                .iter()
                .rev()
                .fold(0u64, |acc, &b| (acc << 8) | b as u64);
            Ok(res)
        } else {
            Err(GdbError::InvalidResponse)
        }
    }

    pub fn difftest_raise_intr(&self, _no: u64) {
        println!("raise_intr is not supported");
        panic!("raise_intr is not supported");
    }
}

impl Drop for DiffTest {
    fn drop(&mut self) {
        if let Some(mut process) = self.qemu_process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}

// Main function example
#[test]
fn diff() -> Result<(), GdbError> {
    let mut difftest = DiffTest::new(1234)?;

    let image: &[u8] = &[
        0x13, 0x01, 0x01, 0xfe, // addi    sp,sp,-32
    ]
    .repeat(10);
    difftest.difftest_memcpy_to(0x80_000_000, image)?;
    let res = difftest.difftest_memcpy_from(0x80_000_000, 12)?;
    println!("Memory read from 0x80_000_000: {:x?}", res);

    // Example: Get/set registers
    let mut regs = vec![0u64; DIFFTEST_REG_SIZE]; // 32 registers for RISC-V
    regs[32] = 0x80_000_000; // set pc
    difftest.difftest_write_general_regs(&mut regs)?;

    for _ in 0..10 {
        difftest.single_step()?;
        let mut regs = vec![0; DIFFTEST_REG_SIZE];
        difftest.difftest_read_general_regs(&mut regs)?;
        println!("SP, PC after step: [{:#x?}], sp: {:x?}", regs[32], regs[2]);
    }

    // mscratch = 0x340,
    // mepc = 0x341,
    // mcause = 0x342,
    // mtval = 0x343,
    let mscratch = difftest.difftest_read_csr_reg(0x340).unwrap_or_default();
    let mepc = difftest.difftest_read_csr_reg(0x341).unwrap_or_default();
    let mcause = difftest.difftest_read_csr_reg(0x342).unwrap_or_default();
    let mtval = difftest.difftest_read_csr_reg(0x343).unwrap_or_default();
    println!(
        "csrs: mscratch: {:#x}, mepc: {:#x}, mcause: {:#x}, mtval: {:#x}",
        mscratch, mepc, mcause, mtval
    );

    Ok(())
}

#[test]
fn breakpoint_test() -> Result<(), GdbError> {
    let mut difftest = DiffTest::new(1235)?;
    let image: &[u8] = &[
        0x13, 0x01, 0x01, 0xfe, // addi    sp,sp,-32
    ]
    .repeat(10);
    difftest.difftest_memcpy_to(0x80_000_000, image)?;
    let mut regs = vec![0u64; DIFFTEST_REG_SIZE]; // 32 registers for RISC-V
    regs[32] = 0x80_000_000; // set pc
    difftest.difftest_write_general_regs(&mut regs)?;

    let breakpoint_addr = 0x80_000_000 + 4 * 5; // Set a breakpoint at this address
    difftest.breakpoint(breakpoint_addr)?;
    difftest.continues()?;
    difftest.rm_breakpoint(breakpoint_addr)?;
    let mut regs = vec![0; DIFFTEST_REG_SIZE];
    difftest.difftest_read_general_regs(&mut regs)?;
    println!(
        "SP, PC after breakpoint: [{:#x?}], sp: {:x?}",
        regs[32], regs[2]
    );

    Ok(())
}
