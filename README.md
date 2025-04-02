Given a chunk of text, as well as a list of ranges to highlight, generate a HTML representation for displaying the search result:

Input:
text: Chunk of text, e.g., “This is a sample text”
ranges: Ranges to highlight, e.g. [5..7, 10..16] corresponds to “is” and “sample”

Output:
output: HTML representation, e.g., “This <em>is</em> a <em>sample</em> text”
