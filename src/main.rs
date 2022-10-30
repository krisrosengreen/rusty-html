// Get rid of warnings derived from macros
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(missing_fragment_specifier)]

#[derive(Clone, Debug)]
struct AttrPair {
    attr_name: String,
    attr_val: String,
}

#[derive(Clone, Debug)]
struct Node {
    node_name: String,
    attrs: Vec<AttrPair>,
}

#[derive(Clone)]
struct HTMLCode {
    nodes: Vec<Node>,
}

macro_rules! html_node {
    ($item_name:ident $($attr_name:ident=$attr_val:literal )*) => {
        {
            let mut attr: Vec<AttrPair> = Vec::new();
            $(attr.push(AttrPair{attr_name: String::from(stringify!($attr_name)), attr_val: String::from($attr_val) });)*

            let node_name = String::from(stringify!($item_name));
            Node {node_name: node_name, attrs: attr.clone()}
        }
    };
}

macro_rules! html {
    ($(<$(/)?$tag:ident $($attr_name:ident=$attr_val:literal )*>)*) => {
        {
            let mut nodes: Vec<Node> = Vec::new();
            $(nodes.push(html_node!($tag $($attr_name=$attr_val )*));)*

            HTMLCode{nodes: nodes}
        }
    };
}

fn main() {
    // An example
    let code1: HTMLCode = html!(

        <html>
            <head>
            </head>
            <body>
                <div class="super_class">
                    <a link="http://www.somewhere.com"></a>
                </div>
            </body>
        </html>

    );

    for i in code1.nodes.iter() {
        println!("{:?}", i);
    }

    // Another example
    let code2: HTMLCode = html!(

        <help type="Common bug">
            <link src="http://www.helphere.com"></link>
        </help>

    );

    for i in code2.nodes.iter() {
        println!("{:?}", i);
    }
}
