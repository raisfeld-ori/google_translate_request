
# google translate request

Google has this url:

https://translate.googleapis.com/translate_a/single?client=gtx&sl=(INSERT ORIGINAL LANGUAGE)&tl=(INSERT TARGET LANGUAGE)&dt=t&q=(INSERT TEXT HERE)

Which is used for translating text. This package is simply a wrapper for that URL.

## usage

```rust
    async fn test_translate() {
        let text = "Hello, world!";
        let from = "en";
        let to = "zh-CN";
        let resp = translate(text, from, to).await.unwrap();
        println!("{}", resp);
    }
```

### extra note

this package is so small that it contains more testing code than actual code.
If you're not using reqwest or if you have any modification needs, it's better to write your own code than use this.

However, this package is useful for anyone with the same need as me, which is a simple translation package that can be learned
in a couple of minutes, and won't require any prior knowlage about google.
