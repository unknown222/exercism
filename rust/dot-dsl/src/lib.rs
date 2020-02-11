

pub mod graph {

    pub mod graph_items {

        pub mod node {

            use std::fmt;
            use std::collections::HashMap;

            #[derive(Clone)]
            pub struct Node {
                pub value: String,
                pub attrs: HashMap<String, String>
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        value: value.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs.into_iter()
                            .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
                            .collect(),
                        ..self
                    }
                }

                pub fn get_attr(self, name: &str) -> Option<String> {
                    Some(self.attrs.get(name)?.to_string())
                }
            }

            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    self.value == other.value
                }
            }

            impl fmt::Debug for Node {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "Node {{ value: {} }}", self.value)
                }
            }
        }

        pub mod edge {

            use std::fmt;
            use std::collections::HashMap;

            #[derive(Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs.into_iter()
                            .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
                            .collect(),
                        ..self
                    }
                }
            }

            impl PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    self.from == other.from &&
                    self.to == other.to
                }
            }

            impl fmt::Debug for Edge {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "Edge {{ from: {}, to: {} }}", self.from, self.to)
                }
            }
        }


    }

    use graph_items::node::Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {

        pub fn new() -> Self {
            return Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            };
        }

        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Graph {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs.into_iter()
                    .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
                    .collect(),
                ..self
            }
        }

        pub fn get_node(self, value: &str) -> Option<Node> {
            Some(self.nodes.into_iter().find(|node| node.value == value.to_string())?)
        }
    }
}
