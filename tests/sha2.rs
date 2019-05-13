extern crate hacl_star;

use hacl_star::sha2;

const INPUT: &[u8] = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu";

#[test]
fn test_sha256() {
    const OUTPUT: [u8; 32] = [
        0xcf, 0x5b, 0x16, 0xa7, 0x78, 0xaf, 0x83, 0x80, 0x03, 0x6c, 0xe5, 0x9e, 0x7b, 0x04, 0x92,
        0x37, 0x0b, 0x24, 0x9b, 0x11, 0xe8, 0xf0, 0x7a, 0x51, 0xaf, 0xac, 0x45, 0x03, 0x7a, 0xfe,
        0xe9, 0xd1,
    ];
    let mut output1 = [0; 32];
    let mut output2 = [0; 32];

    sha2::Sha256::hash(&mut output1, INPUT);
    let mut hasher = sha2::Sha256::default();
    hasher.update(&INPUT[..5]);
    hasher.update(&INPUT[5..][..65]);
    hasher.update(&INPUT[70..]);
    hasher.finish(&mut output2);

    assert_eq!(output1, output2);
    assert_eq!(output1, OUTPUT);
}

#[test]
fn test_sha384() {
    const OUTPUT: [u8; 48] = [
        0x09, 0x33, 0x0c, 0x33, 0xf7, 0x11, 0x47, 0xe8, 0x3d, 0x19, 0x2f, 0xc7, 0x82, 0xcd, 0x1b,
        0x47, 0x53, 0x11, 0x1b, 0x17, 0x3b, 0x3b, 0x05, 0xd2, 0x2f, 0xa0, 0x80, 0x86, 0xe3, 0xb0,
        0xf7, 0x12, 0xfc, 0xc7, 0xc7, 0x1a, 0x55, 0x7e, 0x2d, 0xb9, 0x66, 0xc3, 0xe9, 0xfa, 0x91,
        0x74, 0x60, 0x39,
    ];
    let mut output1 = [0; 48];
    let mut output2 = [0; 48];

    sha2::Sha384::hash(&mut output1, INPUT);
    let mut hasher = sha2::Sha384::default();
    hasher.update(&INPUT[..5]);
    hasher.update(&INPUT[5..][..65]);
    hasher.update(&INPUT[70..]);
    hasher.finish(&mut output2);

    assert_eq!(&output1[..], &output2[..]);
    assert_eq!(&output1[..], &OUTPUT[..]);
}

#[test]
fn test_sha512() {
    const OUTPUT: [u8; 64] = [
        0x8e, 0x95, 0x9b, 0x75, 0xda, 0xe3, 0x13, 0xda, 0x8c, 0xf4, 0xf7, 0x28, 0x14, 0xfc, 0x14,
        0x3f, 0x8f, 0x77, 0x79, 0xc6, 0xeb, 0x9f, 0x7f, 0xa1, 0x72, 0x99, 0xae, 0xad, 0xb6, 0x88,
        0x90, 0x18, 0x50, 0x1d, 0x28, 0x9e, 0x49, 0x00, 0xf7, 0xe4, 0x33, 0x1b, 0x99, 0xde, 0xc4,
        0xb5, 0x43, 0x3a, 0xc7, 0xd3, 0x29, 0xee, 0xb6, 0xdd, 0x26, 0x54, 0x5e, 0x96, 0xe5, 0x5b,
        0x87, 0x4b, 0xe9, 0x09,
    ];
    let mut output1 = [0; 64];
    let mut output2 = [0; 64];

    sha2::Sha512::hash(&mut output1, INPUT);
    let mut hasher = sha2::Sha512::default();
    hasher.update(&INPUT[..5]);
    hasher.update(&INPUT[5..][..65]);
    hasher.update(&INPUT[70..]);
    hasher.finish(&mut output2);

    assert_eq!(&output1[..], &output2[..]);
    assert_eq!(&output1[..], &OUTPUT[..]);
}
