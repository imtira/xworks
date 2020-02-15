pub const CSS: &'static str = "
<style>
html,body {
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

h1 {
    font-size: 200%;
    margin: .67em 0
}
h2 {
    font-size: 180%;
    margin: .75em 0
}
h3 {
    font-size: 150%;
    margin: .83em 0
}
h4 {
    font-size: 130%;
    margin: 1.12em 0
}
h5 {
    font-size: 115%;
    margin: 1.5em 0
}
h6 {
    font-size: 100%;
    margin: 0;
}

sup, sub {
    font-size: 80%;
    line-height: 0;
}
</style>";