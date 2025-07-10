use alloc::vec::Vec;

pub fn screen_width_height() -> (usize, usize) {
    let width_height =
        unsafe { core::ptr::read_volatile(crate::config::VGACTL_MMIO_START as *const u32) };

    let width = (width_height >> 16) as usize;
    let height = (width_height & 0xFFFF) as usize;
    (width, height)
}

pub fn new_frame_buffer() -> Vec<u8> {
    let (width, height) = screen_width_height();
    let bytes_per_pixel = 4; // ARGB8888 format
    vec![0; width * height * bytes_per_pixel]
}

pub fn write_frame(src: &[u8]) {
    let (width, height) = screen_width_height();
    let bytes_per_pixel = 4; // ARGB8888 format
    let expected_size = width * height * bytes_per_pixel;
    let dst = unsafe {
        core::slice::from_raw_parts_mut(crate::config::FB_START as *mut u8, expected_size)
    };
    debug_assert_eq!(
        src.len(),
        dst.len(),
        "Source and destination buffers must have the same length"
    );
    dst.copy_from_slice(src);
}

pub fn write_pixel(x: usize, y: usize, color: u32) {
    let (width, height) = screen_width_height();
    let bytes_per_pixel = 4; // ARGB8888 format
    if x < width && y < height {
        let index = (y * width + x) * bytes_per_pixel;
        let dst = unsafe {
            core::slice::from_raw_parts_mut(
                crate::config::FB_START as *mut u8,
                width * height * bytes_per_pixel,
            )
        };
        dst[index] = (color & 0xFF) as u8; // B
        dst[index + 1] = ((color >> 8) & 0xFF) as u8; // G
        dst[index + 2] = ((color >> 16) & 0xFF) as u8; // R
        dst[index + 3] = ((color >> 24) & 0xFF) as u8; // A
    }
}

pub fn sync_frame() {
    unsafe {
        let sync_ctrl = (crate::config::MMIO_START + 0x100 + 4) as *mut u32;
        core::ptr::write_volatile(sync_ctrl, 0x1);
    }
}


/// 将图像数据绘制到指定区域
/// 
/// # 参数
/// * `x` - 目标区域的左上角 x 坐标
/// * `y` - 目标区域的左上角 y 坐标
/// * `width` - 图像宽度
/// * `height` - 图像高度
/// * `src` - 源图像数据 (ARGB8888 格式)
pub fn draw_area(x: usize, y: usize, width: usize, height: usize, src: &[u8]) {
    let (screen_width, screen_height) = screen_width_height();
    let bytes_per_pixel = 4; // ARGB8888 format
    
    // 边界检查
    if x >= screen_width || y >= screen_height {
        return;
    }
    
    // 计算实际可绘制的区域
    let draw_width = core::cmp::min(width, screen_width - x);
    let draw_height = core::cmp::min(height, screen_height - y);
    
    // 检查源数据大小
    let expected_src_size = width * height * bytes_per_pixel;
    if src.len() < expected_src_size {
        return;
    }
    
    let dst = unsafe {
        core::slice::from_raw_parts_mut(
            crate::config::FB_START as *mut u8,
            screen_width * screen_height * bytes_per_pixel
        )
    };
    
    // 逐行复制像素数据
    for row in 0..draw_height {
        let src_row_start = row * width * bytes_per_pixel;
        let dst_row_start = ((y + row) * screen_width + x) * bytes_per_pixel;
        
        let src_row_end = src_row_start + draw_width * bytes_per_pixel;
        let dst_row_end = dst_row_start + draw_width * bytes_per_pixel;
        
        if src_row_end <= src.len() && dst_row_end <= dst.len() {
            dst[dst_row_start..dst_row_end]
                .copy_from_slice(&src[src_row_start..src_row_end]);
        }
    }
}

/// 清除指定区域
///
/// # 参数
/// * `x` - 区域左上角 x 坐标
/// * `y` - 区域左上角 y 坐标
/// * `width` - 区域宽度
/// * `height` - 区域高度
/// * `color` - 填充颜色 (ARGB8888 格式)
pub fn clear_area(x: usize, y: usize, width: usize, height: usize, color: u32) {
    let (screen_width, screen_height) = screen_width_height();
    let bytes_per_pixel = 4;

    // 边界检查
    if x >= screen_width || y >= screen_height {
        return;
    }

    // 计算实际可清除的区域
    let clear_width = core::cmp::min(width, screen_width - x);
    let clear_height = core::cmp::min(height, screen_height - y);

    let dst = unsafe {
        core::slice::from_raw_parts_mut(
            crate::config::FB_START as *mut u8,
            screen_width * screen_height * bytes_per_pixel,
        )
    };

    // 提取颜色分量
    let b = (color & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let r = ((color >> 16) & 0xFF) as u8;
    let a = ((color >> 24) & 0xFF) as u8;

    // 逐行清除
    for row in 0..clear_height {
        let row_start = ((y + row) * screen_width + x) * bytes_per_pixel;
        let row_end = row_start + clear_width * bytes_per_pixel;

        if row_end <= dst.len() {
            for col in 0..clear_width {
                let pixel_offset = row_start + col * bytes_per_pixel;
                dst[pixel_offset] = b;
                dst[pixel_offset + 1] = g;
                dst[pixel_offset + 2] = r;
                dst[pixel_offset + 3] = a;
            }
        }
    }
}
