pub fn html_start(title: &'_ str) -> String {
    format!(
        "<html>
<head>
    <title>{}</title>
</head>
<div id=\"page\">
</html>",
        title
    )
}

pub const HTML_END: &str = "</div></html>";

pub const CSS: &str = "
<style>
#page {
    margin: 20px;
    padding: 20px;
}

html, body {
	margin:0;
	padding:0;
	border: none;
 	background: transparent;
	font-family: \"Linux Libertine O\", serif;
}

a {
   color:#000000;
   text-decoration: underline;
}

.table-of-contents a {
   text-decoration: none;
}

.table-of-content-title {
    margin: 0;
    font-weight: bold;
    font-size: 100%;
    padding-left: 1em;
}

.table-of-content-subtitle {
    margin: 0;
    font-weight: normal;
    font-size: 90%;
    padding-left: 2em;
}

h1 {
    font-size: 130%;
    margin: 1.12em 0
}
h2 {
    font-size: 115%;
    margin: 1.5em 0
}
h3 {
    font-size: 100%;
    margin: 0;
}

sup, sub {
    font-size: 80%;
    line-height: 0;
}
</style>";
