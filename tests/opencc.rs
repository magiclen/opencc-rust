extern crate opencc_rust;

use opencc_rust::{DefaultConfig, OpenCC};

#[test]
fn tw2sp() {
    let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();
    assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
               &opencc.convert("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。"));
}

#[test]
fn tw2sp_to_buffer() {
    let s = String::from("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。");

    let opencc = OpenCC::new(DefaultConfig::TW2SP).unwrap();
    let s = opencc
        .convert_to_buffer("雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。", s);

    assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。",
               &s);
}

#[test]
fn s2twp() {
    let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();
    assert_eq!("涼風有訊，秋月無邊，虧我思嬌的情緒好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
               &opencc.convert("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。"));
}

#[test]
fn s2twp_to_buffer() {
    let s = String::from("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。");

    let opencc = OpenCC::new(DefaultConfig::S2TWP).unwrap();
    let s = opencc
        .convert_to_buffer("虽然我不是玉树临风，潇洒倜傥，但我有广阔的胸襟，加强劲的臂弯。", s);

    assert_eq!("凉风有讯，秋月无边，亏我思娇的情绪好比度日如年。雖然我不是玉樹臨風，瀟灑倜儻，但我有廣闊的胸襟，加強勁的臂彎。",
               &s);
}
