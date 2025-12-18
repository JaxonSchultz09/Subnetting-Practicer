fn main() 
{
    slint_build::compile("ui/InputFieldGeneric.slint").expect("Slint build failed");
    slint_build::compile("ui/IPClassButtons.slint").expect("Slint build failed");
    slint_build::compile("ui/gui.slint").expect("Slint build failed");
}