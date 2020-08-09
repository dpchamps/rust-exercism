pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.into(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|&(k, v)| {
                        self.attrs.insert(k.to_owned(), v.to_owned());
                    });

                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_ref)
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.into(),
                        to: to.into(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|&(k, v)| {
                        self.attrs.insert(k.to_owned(), v.to_owned());
                    });

                    self
                }
            }
        }
    }

    #[derive(PartialEq, Debug, Default)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());

            self
        }

        pub fn with_edges(mut self, nodes: &[graph_items::edge::Edge]) -> Self {
            self.edges.extend(nodes.iter().cloned());

            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|&(k, v)| {
                self.attrs.insert(k.to_owned(), v.to_owned());
            });

            self
        }

        pub fn get_node(self, node_name: &str) -> Option<graph_items::node::Node> {
            self.nodes
                .into_iter()
                .find(|node| node.name == node_name)
        }
    }
}
