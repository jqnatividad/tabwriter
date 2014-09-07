use std::io;

use TabWriter;

fn ordie<T, E: ToString>(r: Result<T, E>) -> T {
    match r {
        Ok(r) => r,
        Err(e) => fail!("{}", e.to_string()),
    }
}

fn readable_str<S: StrAllocating>(s: S) -> String {
    s.replace("\n", "\\n")
     .replace("\t", "\\t")
     .replace(" ", "·")
}

fn tabw() -> TabWriter<io::MemWriter> {
    TabWriter::new(io::MemWriter::new())
}

fn tabify(mut tw: TabWriter<io::MemWriter>, s: &str) -> String {
    ordie(tw.write_str(s));
    ordie(tw.flush());
    ordie(String::from_utf8(tw.unwrap().unwrap()))
}

fn iseq(tw: TabWriter<io::MemWriter>, s: &str, expected: &str) {
    let written = tabify(tw, s);
    let got = written.as_slice();
    if expected != got {
        let expected = readable_str(expected);
        let got = readable_str(got);
        fail!("expected = '{}' != '{}' = got", expected, got);
    }
}

#[test]
fn test_no_cells() {
    iseq(tabw(), "foo\nbar\nfubar", "foo\nbar\nfubar");
}

#[test]
fn test_no_cells_trailing() {
    iseq(tabw(), "foo\nbar\nfubar\n", "foo\nbar\nfubar\n");
}

#[test]
fn test_no_cells_prior() {
    iseq(tabw(), "\nfoo\nbar\nfubar", "\nfoo\nbar\nfubar");
}

#[test]
fn test_empty() {
    iseq(tabw(), "", "");
}

#[test]
fn test_empty_lines() {
    iseq(tabw(), "\n\n\n\n", "\n\n\n\n");
}

#[test]
fn test_empty_cell() {
    iseq(tabw().padding(0).minwidth(2), "\t\n", "  \n");
}

#[test]
fn test_empty_cell_no_min() {
    iseq(tabw().padding(0).minwidth(0), "\t\n", "\n");
}

#[test]
fn test_empty_cells() {
    iseq(tabw().padding(0).minwidth(2), "\t\t\n", "    \n");
}

#[test]
fn test_empty_cells_no_min() {
    iseq(tabw().padding(0).minwidth(0), "\t\t\n", "\n");
}

#[test]
fn test_empty_cells_ignore_trailing() {
    iseq(tabw().padding(0).minwidth(2), "\t\t\t", "    ");
}

#[test]
fn test_one_cell() {
    iseq(tabw().padding(2).minwidth(2), "a\tb\nxx\tyy", "a   b\nxx  yy");
}

#[test]
fn test_no_padding() {
    iseq(tabw().padding(0).minwidth(2), "a\tb\nxx\tyy", "a b\nxxyy");
}

#[test]
fn test_minwidth() {
    iseq(tabw().minwidth(5).padding(0),
         "a\tb\nxx\tyy", "a    b\nxx   yy");
}

#[test]
fn test_contiguous_columns() {
    iseq(tabw().padding(1).minwidth(0),
         "x\tfoo\tx\nx\tfoofoo\tx\n\nx\tfoofoofoo\tx",
         "x foo    x\nx foofoo x\n\nx foofoofoo x");
}


#[test]
fn test_contiguous_columns_complex() {
    iseq(tabw().padding(1).minwidth(3),
"
fn foobar() {
 	let mut x = 1+1;	// addition
 	x += 1;	// increment in place
 	let y = x * x * x * x;	// multiply!

 	y += 1;	// this is another group
 	y += 2 * 2;	// that is separately aligned
}
",
"
fn foobar() {
    let mut x = 1+1;       // addition
    x += 1;                // increment in place
    let y = x * x * x * x; // multiply!

    y += 1;     // this is another group
    y += 2 * 2; // that is separately aligned
}
");
}
