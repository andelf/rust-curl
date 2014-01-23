#[license = "MIT"];


extern mod curl;

use std::libc::{fopen, fclose};
use std::c_str;
use std::cast;

#[test]
fn test_version() {
    assert!(curl::version().len() > 0)
}

#[test]
fn test_easy_init() {
    let h = curl::easy::init();
    assert!(!curl::easy::init().is_null());
    curl::easy::cleanup(h)
}

#[test]
fn test_easy_curl_init() {
    let c = curl::easy::Curl::init();
    assert!(!c.is_null());
    c.cleanup()
}

#[test]
fn test_easy_curl_perform() {
    let c = curl::easy::Curl::init();
    let ret = c.perform();
    assert!(ret == 3);
    c.cleanup();
}

#[test]
fn test_easy_strerror() {
    assert!(curl::easy::strerror(0) == ~"No error");
    assert!(curl::easy::strerror(3).len() > 0);
}

#[test]
fn test_easy_escape() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.escape("abcEFG"), ~"abcEFG");
    assert_eq!(c.escape("&*()"), ~"%26%2A%28%29");
    // c.escape("\x00fuck"));
    c.cleanup();
}

#[test]
fn test_easy_duphandle() {
    let c = curl::easy::Curl::init();
    assert!(!c.is_null());
    let cc = c.duphandle();
    assert!(!c.is_null());
    c.cleanup();
    cc.cleanup();
}

#[test]
fn test_easy_reset() {
    let c = curl::easy::Curl::init();
    c.reset();
    assert!(!c.is_null());
    c.cleanup();
}

#[test]
fn test_easy_unescape() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.unescape("abcEFG"), ~"abcEFG");
    assert_eq!(c.unescape("%26%2A%28%29"), ~"&*()");
    c.cleanup();
}

#[test]
fn test_easy_setopt_URL() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, "http://baidu.com/"), 0);
    let ret = c.perform();
    assert!(ret == 0 || ret == 7); // OK or cound't connect
    c.cleanup();
}

#[test]
fn test_easy_setopt() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, "http://baidu.com/"), 0);
    assert_eq!(c.setopt(curl::opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);

}

#[test]
fn test_easy_setopt_bytes() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, bytes!("http://www.baidu.com/")), 0);
    assert_eq!(c.setopt(curl::opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);
    c.cleanup();
}

#[test]
fn test_global_init() {
    let ret = curl::global_init(curl::GLOBAL_ALL);
    assert_eq!(ret, 0);
    // curl::global_cleanup()
}

#[test]
fn test_setopt_slist() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, bytes!("http://fledna.duapp.com/headers")), 0);
    c.setopt(curl::opt::HTTPHEADER, ~[~"X-Dummy: just a test."]);
    assert_eq!(c.setopt(curl::opt::VERBOSE, false), 0);
    let ret = c.perform();
    assert_eq!(ret, 0);
    c.cleanup();
}

#[test]
fn test_setopt_writedata() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, bytes!("http://www.baidu.com")), 0);
    let fp = "/tmp/test.out".to_c_str().with_ref(|fname| {
            "w".to_c_str().with_ref(|mode| {
                    unsafe { fopen(fname, mode) }
                })
                });
    c.setopt(curl::opt::WRITEDATA, fp);
    c.setopt(curl::opt::VERBOSE, false);
    c.perform();
    unsafe { fclose(fp) };
}


#[test]
fn test_setopt_progress_function() {
    let c = curl::easy::Curl::init();
    assert_eq!(c.setopt(curl::opt::URL, bytes!("http://curl.haxx.se/download/curl-7.34.0.zip")), 0);
    let func: |f64,f64,f64,f64| -> int = |dltotal, dlnow, ultotal, ulnow| {
        println!("progress func test: {} {} {} {}", dltotal, dlnow, ultotal, ulnow);
        0
    };
    c.setopt(curl::opt::NOPROGRESS, false);
    c.setopt(curl::opt::WRITEFUNCTION, 0);
    let ret = c.setopt(curl::opt::PROGRESSFUNCTION, 0);
    println!("setopt ret={}", ret);
    println!("perform result = {}", c.perform());
}
