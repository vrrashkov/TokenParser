import SwiftUI
public class DSCoreButtonLg {
public static let defaultHeight = CGFloat(20)  // desc = test desc
public static let defaultDissolve = CustomTransition(duration: 0.45,  x1: CGFloat(0.6968395709991455), x2: CGFloat(0.06683959811925888), y1: CGFloat(0.05232666060328483), y2: CGFloat(0.9323266744613647)) 
public static let defaultGradientSingleWithMultipleColorStops = CustomGradient(gradientType: radial, rotation: 180, 
color1: Style1( 
Color(red: 1.000, green: 0.722, blue: 0.000, opacity: 1.000) ,  
Color(red: 1.000, green: 0.541, blue: 0.000, opacity: 1.000) ,  
Color(red: 1.000, green: 0.180, blue: 0.000, opacity: 1.000) ,  
Color(red: 1.000, green: 0.000, blue: 0.000, opacity: 1.000)  ),
color2: Style2(  
Color(red: 255, green: 184, blue: 0, opacity: 184) ,  
Color(red: 255, green: 138, blue: 0, opacity: 138) ,  
Color(red: 255, green: 46, blue: 0, opacity: 46) ,  
Color(red: 255, green: 0, blue: 0, opacity: 0)   ) 
}
