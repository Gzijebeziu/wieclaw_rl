#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum HorizontalPlacement { Left, Center, Right }

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub enum VerticalPlacement { Top, Center, Bottom }

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone)]
pub struct PrefabSection {
    pub template : &'static str,
    pub width : usize,
    pub height: usize,
    pub placement : (HorizontalPlacement, VerticalPlacement)
}

#[allow(dead_code)]
pub const UNDERGROUND_FORT : PrefabSection = PrefabSection{
    template : RIGHT_FORT,
    width: 15,
    height: 43,
    placement: ( HorizontalPlacement::Right, VerticalPlacement::Top )
};

#[allow(dead_code)]
const RIGHT_FORT : &str = "
     #         
  #######      
  #     #      
  #     #######
  # D         #
  #     #######
  #     #      
  ### ###      
    # #        
    # #        
    # ##       
    ^          
    ^          
    # ##       
    # #        
    # #        
    # #        
    # #        
  ### ###      
  #     #      
  #     #      
  #  D  #      
  #     #      
  #     #      
  ### ###      
    # #        
    # #        
    # #        
    # ##       
    ^          
    ^          
    # ##       
    # #        
    # #        
    # #        
  ### ###      
  #     #      
  #     #######
  #  D        #
  #     #######
  #     #      
  #######      
     #         
";

#[allow(dead_code)]
pub const GNOM_CAMP : PrefabSection = PrefabSection{
    template : GNOM_CAMP_TXT,
    width: 12,
    height: 12,
    placement: ( HorizontalPlacement::Center, VerticalPlacement::Center )
};

#[allow(dead_code)]
const GNOM_CAMP_TXT : &str = "
            
 ≈≈≈≈G≈≈≈≈≈ 
 ≈☼      ☼≈ 
 ≈ g      ≈ 
 ≈        ≈ 
 ≈    g   ≈ 
 G   A    G 
 ≈        ≈ 
 ≈ g      ≈ 
 ≈     g  ≈ 
 ≈☼      ☼≈ 
 ≈≈≈≈G≈≈≈≈≈ 
            
";

#[allow(dead_code)]
pub const BUHAJ_ENTRY : PrefabSection = PrefabSection{
  template : BUHAJ_ENTRY_TXT,
  width: 12,
  height: 10,
  placement: ( HorizontalPlacement::Center, VerticalPlacement::Center )
};

#[allow(dead_code)]
const BUHAJ_ENTRY_TXT : &str = "
            
 ########## 
 #        # 
 #   >    # 
 #        # 
 #B       # 
    B     # 
 #B       # 
 ########## 
            
";