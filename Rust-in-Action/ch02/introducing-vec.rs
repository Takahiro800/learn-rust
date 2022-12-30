fn main() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop.
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the ssame with books.
What do we seek
through millins of pages?";

    // tagsに、マッチした各行の番号を入れる
    let mut tags: Vec<usize> = vec![];

    // ctxに、コンテクスト行を格納したマッチごとのベクターを入れる
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    // linesのイテレーションで、マッチがあった行番号を記録
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        // ref lineはこの値を委譲ではなく、借用したいと伝える
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

/*
NOTE
- saturating_sub()という減算は、整数のアンダーフローが発生するとき、プログラムをクラッシュする代わりに０を返す

- Vec<T>が最高の性能を発揮するのは、Vec::with_capacity()を介してサイズのヒントを与えることで、OSからメモリを割り当てる回数を少なくできる

- ネイティブなテキストファイルをStringに読み込むと、無効なバイトの際にクラッシュする。 『実践Rustプログラミング入門』 p.186を参照
*/
