use themes::Colorcode;
use themes::Theme;
use themes::Symbols;

//               data  , FG       , BG        
type _Fragment = (String, Colorcode, Colorcode);
type _FragmentChain = Vec<_Fragment>;

pub struct Prompt {
    pub theme: Theme,
    pub symbols: Symbols,
   
}
