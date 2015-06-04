use megam_rustyprint::Printer;

#[test]
fn print() {

    let mut a = Printer::new();
    let mut b = Vec::new();
    b.push("header1".to_string());
    b.push("header2dfhfghfghg".to_string());
 		b.push("header3".to_string());
 	//	b.push("header5".to_string());
    a.set_header(b);

    let mut parent = Vec::new();

    let mut child1 = Vec::new();
    child1.push("namename".to_string());
    child1.push("addressdfgfgghgh".to_string());
		child1.push("email".to_string());

    let mut child2 = Vec::new();
    child2.push("name".to_string());
    child2.push("address".to_string());
		child2.push("emailhhgjghjghj".to_string());

    let mut child3 = Vec::new();
    child3.push("name".to_string());
    child3.push("address".to_string());
		child3.push("email".to_string());

    parent.push(child1);
		parent.push(child2);
		parent.push(child3);

    a.set_body(parent);

		println!("{:?}", a);
   
}
