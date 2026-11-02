pub fn width(c:char)->Option<f32> {Some(match c {
'0'=>3.286196,
'1'=>1.643098,
'2'=>3.286196,
'3'=>3.286196,
'4'=>3.286196,
'5'=>3.286196,
'6'=>3.286196,
'7'=>3.286196,
'8'=>3.286196,
'9'=>3.286196,
'.'=>1.026936,
_=>return None})}
pub fn positions(text:&str)->Vec<(char,f32)> {
let glyphs:Vec<_>=text.chars().filter_map(|c|width(c).map(|w|(c,w))).collect();
let total=glyphs.iter().map(|x|x.1).sum::<f32>()+0.25*glyphs.len().saturating_sub(1) as f32;
let mut pen=-total/2.0;glyphs.into_iter().map(|(c,w)|{let x=pen+w/2.0;pen+=w+0.25;(c,x)}).collect()
}