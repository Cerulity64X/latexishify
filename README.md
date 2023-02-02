# Latex-ishify
An extremely simple program that converts plain text into Latex-like mathematical symbols.
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
* The letter 'h' is not included in the Italic section of the Unicode mathematical character block, it is instead replaced with a sans-serif italicized version, '𝘩'. I do not and cannot reason why, and am beyond confusion.
