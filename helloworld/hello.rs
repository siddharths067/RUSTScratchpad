fn main(){
	let name:String = String::from("Siddharth Singh");
	println!("Hello {}, How are you?", name);
	//process_datatype(name);	
	println!("{}",name);
}
/*fn process_datatype<T>(variable:T){
	match TypeId::of::<T>(){
		TypeId::of::<String>() => {
			println!("The Variable is a String");
		},
		TypeId::of::<Integer>() => {
			println!("The Variable is a Integer");
		}	
	}		
} */
