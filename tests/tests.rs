extern crate liquery;
extern crate liquery_taglib;


macro_rules! taglib_test_single {
    ($name: ident, $path: expr, $field: expr, $out: expr) => {
        #[test]
        fn $name() {
            let queryable = AudioFile::new($path).unwrap();

            assert_eq!($out, &queryable.query($field).unwrap());
        }
    };
}

macro_rules! taglib_test {
    ($name: ident, $field: expr, $out: expr) => {
        mod $name {
            use liquery::Queryable;
            use liquery_taglib::AudioFile;

            taglib_test_single!(flac, "tests/test.flac", $field, $out);
            taglib_test_single!(ogg, "tests/test.ogg", $field, $out);
            taglib_test_single!(mp3, "tests/test.mp3", $field, $out);
            taglib_test_single!(wav, "tests/test.wav", $field, $out);
            taglib_test_single!(wma, "tests/test.wma", $field, $out);
        }
    };
}

taglib_test!(title, "title", "test");
taglib_test!(artist, "artist", "liquery");
taglib_test!(album, "album", "file");
taglib_test!(year, "year", "2016");
