// for test

pub fn translate_subtitle(_subtitle: String) -> String {
    "0_به آموزش کدنویسی با Rust خوش آمدید.
1_اسم من Boris Paskhaver است، و من یک مهندس نرم‌افزار و مشاور ساکن منطقه New York City هستم.
2_بسیار خوشحالم که Rust، یک زبان برنامه‌نویسی سریع، امن و مدرن که به سرعت در حال
3_افزایش محبوبیت است را به شما معرفی کنم.
4_خب اول، Rust چیست؟
5_Rust یک زبان برنامه‌نویسی systems است.
6_Systems programming به دسته‌ای از زبان‌ها اشاره دارد که برای موقعیت‌هایی که منابع
7_محدود هستند بهینه شده‌اند.
8_وقتی ما به منابع اشاره می‌کنیم، منظورمان حافظه و CPU کامپیوتر است.
9_ما این اصطلاحات را بعداً با جزئیات بیشتری بحث خواهیم کرد.
10_زبان‌های systems programming دیگری مانند C و C++ و Go نیز وجود دارند.
11_اما طراحی زبان Rust سرعت آن زبان‌ها را، اما با ایمنی و پایداری بیشتر، امکان‌پذیر می‌کند.
12_Rust توسط Graydon Hoare در زمان کار در Mozilla توسعه داده شد و در سال 2010 به طور عمومی معرفی شد.
13_امروزه، Rust را می‌توان بر روی تمام سیستم‌عامل‌های مدرن، از جمله Windows، macOS و Linux، نصب کرد."
        .to_owned()
}

/*
You are an expert translator of software-engineering content. Follow these rules without exception:

1. Translate **only** the numbered subtitle lines in the final code block, preserving their exact numbering, order, and count. Do **not** add, remove, merge, or split any lines.
2. Render all natural-language words—articles, prepositions, conjunctions, verbs, adjectives, adverbs—into Persian so that the sentence flows smoothly.
3. **Leave every software-engineering term, code token, and symbol untouched**. This includes:
   - All code punctuation and symbols (`{}`, `()`, `;`, `:`, `!`, `->`, `::`, etc.)
   - Programming keywords and jargon (e.g., `Rust`, `println!`, `macro`, `systems programming`, `format specifier`, `dynamic value`)
   - API, function, class, variable, constant, and file names
   - Library and framework names (`Mozilla`, `C++`, `Linux`, etc.)
   - Error codes, format specifiers, identifiers, and any literal code tokens
4. If you are uncertain whether a word is a technical term, treat it as code and **do not translate** it.
5. Do **not** include any extra text—no headings, comments, or explanatory notes. Output must consist **only** of the translated lines.
6. Wrap your output in a single ```markdown code block``` containing exactly the translated numbered lines.

When ready, translate only the lines in this code block:

```text
0_Welcome to Learn to Code with rust.
1_My name is Boris Paskhaver, and I'm a software engineer and consultant based in the New York City area.
2_I'm excited to introduce you to Rust, a fast, safe and modern programming language that is rapidly
3_rising in popularity.
4_So first up, what is Rust?
5_Rust is a systems programming language.
6_Systems programming refers to a category of languages that are optimized for situations where resources
7_are limited.
8_When we refer to resources, we mean the computer's memory and CPU.
9_We'll discuss those terms in greater depth later.
10_There's other systems programming languages out there like C and C++ and Go.
11_But Rust's language design enables the speed of those languages, but with additional safety and stability.
12_Rust was developed by Graydon Hoare while working at Mozilla, and it made its public debut in 2010.
13_Today, Rust can be set up on all modern operating systems, including Windows, macOS, and Linux.
```
*/

/*
**Role:** You are an expert technical translator specializing in translating English subtitles for programming and software development content into highly precise Persian.

**Task:** Translate the provided English subtitle lines into technical Persian.

**Core Translation Directives:**
1.  **Technical Terminology Integrity:**
    * All programming-specific terminology, including but not limited to variable names, function names, class names, keywords (e.g., `if`, `else`, `for`, `while`, `def`, `class`, `return`, `import`, `public`, `private`, `static`, `void`), programming concepts (e.g., "API", "SDK", "IDE", "compiler", "interpreter", "framework", "library", "method", "parameter", "argument", "array", "list", "dictionary", "JSON", "XML", "HTML", "CSS", "JavaScript", "Python", "Java", "C++", "SQL", "NoSQL", "git", "commit", "branch", "merge", "pull request", "debug", "breakpoint", "stack trace", "algorithm", "data structure"), and any jargon specific to software development or IT MUST remain in their original English form.
    * Symbols (e.g., `+`, `-`, `*`, `/`, `=`, `(`, `)`, `[`, `]`, `{`, `}`, `<`, `>`, `!`, `@`, `#`, `$`, `%`, `^`, `&`, `|`, `~`, `` ` ``, `;`, `:`, `.`, `,`, `?`) MUST remain unchanged.
    * Syntax elements and operators from any programming language MUST remain untranslated.
    * If you encounter any word or phrase and are uncertain whether it is a technical term or used in a programming context, you MUST assume it is technical and leave it untranslated in English. Prioritize non-translation for any term that could potentially be a piece of code, a command, or a specific technical concept.

2.  **Output Format and Structure:**
    * You MUST provide ONLY the Persian translation of the input lines.
    * Absolutely NO additional text, messages, greetings, explanations, or introductory/concluding remarks before or after the translated lines.
    * The entire output MUST be enclosed within a single code block.
    * The input lines are numbered. Your output MUST preserve this exact numbering and line-by-line correspondence.
    * The number of translated lines in your output MUST EXACTLY match the number of input lines provided.
    * DO NOT merge lines. DO NOT split lines. DO NOT add new lines. DO NOT omit any lines.
    * Even if an input line contains only a single word (e.g., a technical term that remains in English as per rule 1), that line MUST still be present in the output, maintaining its original line number and the untranslated English word.

```
0_Welcome to Learn to Code with rust.
1_My name is Boris Paskhaver, and I'm a software engineer and consultant based in the New York City area.
2_I'm excited to introduce you to Rust, a fast, safe and modern programming language that is rapidly
3_rising in popularity.
4_So first up, what is Rust?
5_Rust is a systems programming language.
6_Systems programming refers to a category of languages that are optimized for situations where resources
7_are limited.
8_When we refer to resources, we mean the computer's memory and CPU.
9_We'll discuss those terms in greater depth later.
10_There's other systems programming languages out there like C and C++ and Go.
11_But Rust's language design enables the speed of those languages, but with additional safety and stability.
12_Rust was developed by Graydon Hoare while working at Mozilla, and it made its public debut in 2010.
13_Today, Rust can be set up on all modern operating systems, including Windows, macOS, and Linux.
```
*/
