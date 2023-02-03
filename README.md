# Latex-ishify
An extremely simple program that converts plain text into Latex-like mathematical symbols.

# Usage
This can be built as a library or as an executable that acts like `cat`, reading from argv (else stdin) and writing to stdout.\
The latest builds (windows exe and rlib) are in the Releases.\
Directions for running from the CLI:\
Running without passing anything through will echo anything entered in Latex-ish.\
Running with arguments (`latexishify.exe "hello there"`) will print the output similar to echo.\
Running by piping (`echo hello there | latexishify.exe`) will print the output similar to cat.

# Conversions
Can convert Latin characters, Greek characters, and numeric characters. The exact character ranges are as follows:
* a-z
* A-Z
* α-ω
* Α-Ω
* 0-9

# Limitations
The two limitations for this program:\
* Numeric characters are not italicized (and, on second thought, shouldn't be)
* The letter 'h' is not included in the Italic section of the Unicode mathematical character block. I do not and cannot reason why, and am beyond confusion. It is instead replaced with symbol for the Planck constant (in the Letterlike block), 'ℎ'.

# Changelog
* v1.0: Initial release\
* v1.1: Changed '𝘩' to 'ℎ' (credit: bubbliterally on Tumblr)
