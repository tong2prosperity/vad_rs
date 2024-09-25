// use jni::JNIEnv;
// use jni::objects::{JClass, JByteArray};
// use jni::sys::{jbyteArray, jsize};

// // 假设你已经有一个 JNIEnv 实例
// fn convert_jbytearray_to_i16_vec(env: &JNIEnv, byte_array: JByteArray) -> Vec<i16> {
//     // 获取字节数组的长度
//     let len = env.get_array_length(byte_array).expect("Failed to get array length").into();

//     // 获取字节数组的指针
//     let byte_array_ptr: *mut i8 = env.get_byte_array_elements(byte_array, jni::objects::ReleaseMode::NoCopyBack).unwrap();

//     // 将字节数组转换为 Vec<i16>
//     let mut result = Vec::with_capacity((len / 2) as usize);
//     for i in 0..(len / 2) {
//         let byte1 = unsafe { *byte_array_ptr.offset(i as isize * 2) } as i16;
//         let byte2 = unsafe { *byte_array_ptr.offset(i as isize * 2 + 1) } as i16;
//         let value = (byte1 << 8) | byte2;
//         result.push(value);
//     }

//     // 释放字节数组的指针
//     env.release_byte_array_elements(byte_array, byte_array_ptr, jni::objects::ReleaseMode::NoCopyBack).unwrap();

//     result
// }