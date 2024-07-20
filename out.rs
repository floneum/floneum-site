fn main() {
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 0usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "introduction",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#introduction",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Introduction",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm is a library with dead simple interfaces for local, language, audio, and image models"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "quick-start",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#quick-start",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Quick Start",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Add the Kalosm library",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">cargo add kalosm\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Initialize a Kalosm model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> _model = Llama::new_chat();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Start a chat session with a pirate",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> model = Llama::new_chat();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// New code\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> chat = Chat::builder(&amp;</span><span style=\"color:#b48ead;\">mut</span><span style=\"color:#c0c5ce;\"> model)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">with_system_prompt</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">The assistant will act like a pirate</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">loop </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">        chat.</span><span style=\"color:#96b5b4;\">add_message</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#96b5b4;\">prompt_input</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">&gt; </span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">())\n</span><span style=\"color:#c0c5ce;\">            .await\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            .await\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    }\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[
                                    dioxus_core::TemplateNode::Text {
                                        text: "Add build configuration to your ",
                                    },
                                    {
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: ".cargo/config.toml",
                                            }],
                                        }
                                    },
                                    dioxus_core::TemplateNode::Text {
                                        text: " for improved performance",
                                    },
                                ],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">[build]\n</span><span style=\"color:#c0c5ce;\">rustflags = [&quot;</span><span style=\"color:#a3be8c;\">-C</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">target-cpu=native</span><span style=\"color:#c0c5ce;\">&quot;]\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">[target.x86_64-apple-darwin]\n</span><span style=\"color:#c0c5ce;\">rustflags = [&quot;</span><span style=\"color:#a3be8c;\">-C</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">target-feature=-avx,-avx2</span><span style=\"color:#c0c5ce;\">&quot;]\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Run the program",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">cargo run --release\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 32usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "text-recognition",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#text-recognition",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Text Recognition",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm allows developers to extract text information from images using optical character recognition (OCR). This guide demonstrates how to perform single-line OCR using Kalosm's vision module."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "adding-dependencies",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#adding-dependencies",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Adding dependencies",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Before we get started, we need to add an additional crate for image loading. Add the following line to your  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Cargo.toml" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : " file:" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">[dependencies]\n</span><span style=\"color:#c0c5ce;\"># Your Kalosm dependency added in the start of the documentation...\n</span><span style=\"color:#c0c5ce;\">image = &quot;</span><span style=\"color:#a3be8c;\">0.24.7</span><span style=\"color:#c0c5ce;\">&quot;\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-an-ocr-model",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-an-ocr-model",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating an OCR Model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text { text : "Kalosm's  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "vision" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " module provides functionality for text recognition in images. In this example, the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Ocr::builder()" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method is used to create an OCR model that can transcribe single lines of text."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::vision::*;\n</span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> model = Ocr::builder().</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">().</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "loading-image",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#loading-image",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Loading Image",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Next, we need to load an image that contains text. The  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: "image" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " crate provides the ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://docs.rs/image/latest/image/fn.open.html",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "open" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " method to load an image from a file path, or the ",
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://docs.rs/image/latest/image/io/struct.Reader.html",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Reader" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " for more advanced loading options.",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::vision::*;\n</span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> model = Ocr::builder().</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">().</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Replace the file path with the location of your image. This loaded image will be processed for text recognition."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "text-recognition",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#text-recognition",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Text Recognition",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "Finally, we can use the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "recognize_text" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method to extract text information from the image. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "recognize_text" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method takes an  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "OcrInferenceSettings" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct as input. This struct contains the image to be processed, as well as other settings that can be used to customize the OCR process."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> text = model\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">recognize_text</span><span style=\"color:#c0c5ce;\">(OcrInferenceSettings::new(image).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">())\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">println!(&quot;</span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, text);\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This example provides a basic structure for performing single-line OCR using Kalosm's vision module. You can combine text recognition with an "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./llms",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "LLM" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " to analyze complex documents or photos." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 64usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "image-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#image-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Image Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm supports image generation from text descriptions. This guide demonstrates the basic steps for performing image generation using Kalosm's vision module."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "kalosms-image-generation-module",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#kalosms-image-generation-module",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Kalosm's Image Generation Module",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm's vision module includes models for image generation. In this example, the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Wuerstchen::builder()" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method is used to create an image generation model. You can pass options to the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "builder" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method to customize the model." }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::vision::*;\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = Wuerstchen::builder().</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">().</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "setting-generation-parameters",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#setting-generation-parameters",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Setting Generation Parameters",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Next, we need to set the generation parameters. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "WuerstchenInferenceSettings" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct is used to create the generation parameters. We will set the prompt and number of steps for the generation process. The more steps you use, the more detailed the generated image will be (up to the resolution of the output image)."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> settings = WuerstchenInferenceSettings::new(\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">a cute cat with a hat in a room covered with fur with incredible detail</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">.</span><span style=\"color:#96b5b4;\">with_n_steps</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">2</span><span style=\"color:#c0c5ce;\">);\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "performing-image-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#performing-image-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Performing Image Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "Finally, we can use the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "run" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method to generate images based on the given description. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "run" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method takes the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "WuerstchenInferenceSettings" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct we created as input and returns a list of generated images. Then we can iterate through the generated images and save them to the file system."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> images = model.</span><span style=\"color:#96b5b4;\">run</span><span style=\"color:#c0c5ce;\">(settings).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#b48ead;\">for </span><span style=\"color:#c0c5ce;\">(i, img) in images.</span><span style=\"color:#96b5b4;\">iter</span><span style=\"color:#c0c5ce;\">().</span><span style=\"color:#96b5b4;\">enumerate</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    img.</span><span style=\"color:#96b5b4;\">save</span><span style=\"color:#c0c5ce;\">(&amp;format!(&quot;</span><span style=\"color:#d08770;\">{}</span><span style=\"color:#a3be8c;\">.png</span><span style=\"color:#c0c5ce;\">&quot;, i)).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This guide demonstrates the basic steps for performing image generation using Kalosm's vision module with the Wuerstchen model. You can combine image generation with an "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./llms",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "LLM" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " to generate images based on descriptions that the LLM generates to automatically generate reliant images for a situation or automatically improve prompts."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 96usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "audio-transcription",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#audio-transcription",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Audio Transcription",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm provides helpers that allow you to easily transcribe audio data from either your microphone or a file. In this chapter, we will learn how to use Kalosm to perform real time audio transcription."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-a-transcription-model",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-a-transcription-model",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating a Transcription Model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "First, we need to create a transcription model. A transcription model is a machine learning model that can be used to transcribe audio data. Kalosm provides a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Whisper" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct that can be used to create a transcription model."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::{audio::*, language::*};\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create a new whisper model.\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = Whisper::default();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "recording-audio",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#recording-audio",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Recording Audio",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Next, we need to record some audio from our environment. We can use the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "MicInput" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " struct with the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "record_until" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method to record audio from our microphone until a certain point in time:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">tokio::time::{Duration, Instant};\n</span><span style=\"color:#65737e;\">// Record audio from the microphone for 5 seconds.\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> audio = MicInput::default()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">record_until</span><span style=\"color:#c0c5ce;\">(Instant::now() + Duration::from_secs(</span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">))\n</span><span style=\"color:#c0c5ce;\">    .await\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "transcribing-audio",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#transcribing-audio",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Transcribing Audio",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "Finally, we can use the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "transcribe" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method to transcribe the audio data that we recorded into text. The transcribe method takes some audio data and returns a stream of snippets of text along with the confidence of the transcription."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#65737e;\">// Transcribe the audio.\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> transcribed = model.</span><span style=\"color:#96b5b4;\">transcribe</span><span style=\"color:#c0c5ce;\">(audio).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// As the model transcribes the audio, print the text to the console.\n</span><span style=\"color:#c0c5ce;\">transcribed.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "In this chapter, we learned how to use Kalosm to transcribe audio data from our microphone. Audio data can be a powerful source of real time information. You can combine audio data with "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "../guides/retrieval_augmented_generation",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "retrieval-augmented generation" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " to create a " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/live_qa.rs",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "chat bot that understands it's surroundings" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : "." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 128usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "web-crawling",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#web-crawling",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Web Crawling",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Web crawling with Kalosm allows developers to systematically browse the web, extract information from websites, and perform various actions on web pages. In this example, we'll explore the core concepts behind web crawling using Kalosm."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "kalosms-web-crawling-api",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#kalosms-web-crawling-api",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Kalosm's Web Crawling API",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm provides a powerful web crawling API through the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Page::crawl" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method. This method takes a starting URL, a browsing mode, and a closure that defines the crawling logic. The browser mode controls if the crawler should only retrieve the HTML content of the page or if it should also run a full headless browser to execute JavaScript and load dynamic content. The closure defines the behavior for each visited page. It receives a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Page" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " object representing the current web page. The closure must return a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Pin<Box<dyn Future<Output = CrawlFeedback>>>" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " to instruct the crawler to follow links on the current page."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">Page::crawl(\n</span><span style=\"color:#c0c5ce;\">    Url::parse(&quot;</span><span style=\"color:#a3be8c;\">https://floneum.com/blog/floneum_0_2</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">(),\n</span><span style=\"color:#c0c5ce;\">    BrowserMode::Static,\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">|_page: Page| {\n</span><span style=\"color:#c0c5ce;\">        Box::pin(async </span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">{ CrawlFeedback::follow_all() })\n</span><span style=\"color:#c0c5ce;\">            as std::pin::Pin&lt;Box&lt;dyn std::future::Future&lt;Output = CrawlFeedback&gt;&gt;&gt;\n</span><span style=\"color:#c0c5ce;\">    },\n</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">.await\n</span><span style=\"color:#c0c5ce;\">.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "reading-the-html-content-of-a-page",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#reading-the-html-content-of-a-page",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Reading the HTML Content of a Page",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm provides utilities to extract information from the HTML content of a page. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Page::article" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method extracts an article from a webpage. From that article you can extract the title, and the text of the page. If you need lower level access to the HTML content, you can use the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Page::html" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text : " method to get the raw HTML content of the page."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">Page::crawl(\n</span><span style=\"color:#c0c5ce;\">    Url::parse(&quot;</span><span style=\"color:#a3be8c;\">https://floneum.com/blog/floneum_0_2</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">(),\n</span><span style=\"color:#c0c5ce;\">    BrowserMode::Static,\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">|page: Page| {\n</span><span style=\"color:#c0c5ce;\">        Box::pin(async </span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">            println!(&quot;</span><span style=\"color:#a3be8c;\">URL: </span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, page.</span><span style=\"color:#96b5b4;\">url</span><span style=\"color:#c0c5ce;\">());\n</span><span style=\"color:#c0c5ce;\">            println!(&quot;</span><span style=\"color:#a3be8c;\">HTML: </span><span style=\"color:#d08770;\">{:?}</span><span style=\"color:#c0c5ce;\">&quot;, page.</span><span style=\"color:#96b5b4;\">html</span><span style=\"color:#c0c5ce;\">().await);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">            </span><span style=\"color:#b48ead;\">let </span><span style=\"color:#c0c5ce;\">Ok(page) = page.</span><span style=\"color:#96b5b4;\">article</span><span style=\"color:#c0c5ce;\">().await </span><span style=\"color:#b48ead;\">else </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">                </span><span style=\"color:#b48ead;\">return </span><span style=\"color:#c0c5ce;\">CrawlFeedback::follow_none();\n</span><span style=\"color:#c0c5ce;\">            };\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">            println!(&quot;</span><span style=\"color:#a3be8c;\">Title: </span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, page.</span><span style=\"color:#96b5b4;\">title</span><span style=\"color:#c0c5ce;\">());\n</span><span style=\"color:#c0c5ce;\">            println!(&quot;</span><span style=\"color:#a3be8c;\">Article:</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, page.</span><span style=\"color:#96b5b4;\">body</span><span style=\"color:#c0c5ce;\">());\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">            CrawlFeedback::follow_all()\n</span><span style=\"color:#c0c5ce;\">        })\n</span><span style=\"color:#c0c5ce;\">            as std::pin::Pin&lt;Box&lt;dyn std::future::Future&lt;Output = CrawlFeedback&gt;&gt;&gt;\n</span><span style=\"color:#c0c5ce;\">    },\n</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">.await\n</span><span style=\"color:#c0c5ce;\">.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This example serves as a foundational guide for building web crawling applications with Kalosm. You can combine a web crawler with a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./llms",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "LLM" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " to perform more complex analysis on the content of web pages. Or you could train a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/blob/main/interfaces/kalosm-learning/examples/classify.rs",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "custom classifier" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " to classify web pages based on their content." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 160usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "image-segmentation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#image-segmentation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Image Segmentation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "Kalosm supports image segmentation with the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "SegmentAnything" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " model. This guide demonstrates the basic steps for performing image segmentation using Kalosm's vision module."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "kalosms-image-segmentation-module",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#kalosms-image-segmentation-module",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Kalosm's Image Segmentation Module",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm's vision module provides functionality for image segmentation. In this example, the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "SegmentAnything::builder()" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method is used to create an image segmentation model. The builder method lets you set options to customize the model, but in this example, we will use the default options."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::vision::*;\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = SegmentAnything::builder().</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">().</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "loading-image",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#loading-image",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Loading Image",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Next, we need to load an image that contains text. The  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: "image" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " crate provides the ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://docs.rs/image/latest/image/fn.open.html",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "open" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " method to load an image from a file path, or the ",
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://docs.rs/image/latest/image/io/struct.Reader.html",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Reader" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " for more advanced loading options.",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> image = image::open(&quot;</span><span style=\"color:#a3be8c;\">examples/landscape.jpg</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Replace the file path with the location of your image. The loaded image will be segmented to extract a region around a point of interest."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-segmentation-settings",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-segmentation-settings",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating Segmentation Settings",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Next we need to create the settings for the image. After adding the image, we can set the goal point for segmentation. The goal point is the point of interest in the image. The segmentation model will extract the region around this point automatically. In this example, we will set the goal point to the center of the image."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> x = image.</span><span style=\"color:#96b5b4;\">width</span><span style=\"color:#c0c5ce;\">() / </span><span style=\"color:#d08770;\">2</span><span style=\"color:#c0c5ce;\">;\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> y = image.</span><span style=\"color:#96b5b4;\">height</span><span style=\"color:#c0c5ce;\">() / </span><span style=\"color:#d08770;\">2</span><span style=\"color:#c0c5ce;\">;\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> settings = SegmentAnythingInferenceSettings::new(image)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">add_goal_point</span><span style=\"color:#c0c5ce;\">(x, y);\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "performing-segmentation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#performing-segmentation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Performing Segmentation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "Finally, we can use the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "segment_from_points" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text : " method to perform segmentation on the image. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "segment_from_points" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method takes the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "SegmentAnythingInferenceSettings" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct we created as input and returns an mask image of the segment the model extracted. Then we can save the mask image to the file system."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> images = model.</span><span style=\"color:#96b5b4;\">segment_from_points</span><span style=\"color:#c0c5ce;\">(settings).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">images.</span><span style=\"color:#96b5b4;\">save</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">out.png</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This example demonstrates the basic steps for performing image segmentation using Kalosm's vision module."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 192usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "text-generation-with-kalosm-in-rust",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#text-generation-with-kalosm-in-rust",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Text Generation with Kalosm in Rust",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "introduction",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#introduction",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Introduction",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm, provides both synchronous and streaming approaches for generating text. This chapter explores both methods, offering flexibility based on your specific requirements."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "setting-up-the-kalosm-model",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#setting-up-the-kalosm-model",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Setting up the Kalosm Model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Initializing the Kalosm model is the first step in both synchronous and streaming text generation. In this example, we use the model Mistral 7b, but Kalosm also supports other Llama models, Phi, and remote models with the same API."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#65737e;\">// Create a builder for a chat model\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = Llama::builder()\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Set the source of the model\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_source</span><span style=\"color:#c0c5ce;\">(LlamaSource::mistral_7b())\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Build the model. This will fetch the model from the source if it is not cached.\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "synchronous-text-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#synchronous-text-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Synchronous Text Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "For synchronous text generation, use the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "generate_text" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method. Specify the prompt and any additional configurations, such as the maximum length of the generated text:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> text = model\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">generate_text</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">The capital of France is</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Set any options you need\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_max_length</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">300</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Once you are done, call `await` to get the generated text\n</span><span style=\"color:#c0c5ce;\">    .await\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">println!(&quot;</span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, text);\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "streaming-text-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#streaming-text-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Streaming Text Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Streaming text generation allows efficient handling of large amounts of generated text. Use the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "stream_text" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method to initiate the generation process and iterate over the generated tokens:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> text_stream = model\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">stream_text</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">The capital of France is</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Set any options you need\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_max_length</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">1000</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Call `await` to get a stream of text tokens\n</span><span style=\"color:#c0c5ce;\">    .await\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Pipe the generated text to stdout or read individual tokens with the `next` method from the stream ext trait\n</span><span style=\"color:#c0c5ce;\">text_stream.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Just like the synchronous method, you can specify additional configurations, such as the maximum length of the generated text before awaiting the stream to start generating text."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This chapter demonstrated both synchronous and streaming approaches to text generation using Kalosm in Rust. The synchronous method is straightforward and suitable for smaller text generation tasks, while the streaming method efficiently handles larger amounts of generated text. Experiment with different prompts, configurations, and data sources to customize the generated text to your needs. Explore the Kalosm documentation for additional features and options available in the framework. Happy text generation with Kalosm!"
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "If you need even more control over the text generation process, check out the next chapter on the Kalosm API: "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "llms/./structured_generation",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "structured generation" }],
                    }
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 224usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "constrained-generation-in-kalosm",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#constrained-generation-in-kalosm",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Constrained Generation in Kalosm",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "overview",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#overview",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Overview" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Language models can be incredibly powerful tools for difficult to define tasks. However, in some cases, it is necessary to constrain the output of a language model to a specific pattern. For example, you may want to generate text in a JSON format. Kalosm provides a powerful mechanism for constrained generation that allows you to define a set of constraints and generate text that adheres to those constraints."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "defining-constraints",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#defining-constraints",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Defining Constraints",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm provides a set of parsers that can be used to define constraints. These parsers can be combined to create complex constraints. The following parsers are available:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "LiteralParser",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: ": Matches a literal string.",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "IntegerParser" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            ": Matches an integer (along with parsers for each rust integer type)."
                        }],
                    }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "FloatParser",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: ": Matches a float.",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "StringParser",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: ": Matches a string.",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "SeparatorParser" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            ": Matches any number of items separated by a separator."
                        }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "IndexParser" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            ": Matches any of a set of parsers and returns the index of the matched parser."
                        }],
                    }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "StopOn",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: ": Matches anything until a literal.",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "WordParser",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: ": Matches a single word.",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "VecParser",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: ": Matches a vector of items.",
                                        },
                                    ],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "These parsers can be combined using the following combinators:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "then" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            ": Matches the first parser followed by the second parser."
                        }],
                    }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "or",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text:
                                                ": Matches the first parser or the second parser.",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::code::TAG_NAME,
                                                namespace: dioxus_elements::code::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "repeat",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text:
                                                ": Matches the parser a specified number of times.",
                                        },
                                    ],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "In this example, we will create a parser that completes a sentence with only valid states by combining the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "LiteralParser" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : " and  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "IndexParser" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ":" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create a list of parser for states\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> states = [&quot;</span><span style=\"color:#a3be8c;\">Alaska</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Delaware</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Florida</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Georgia</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Hawaii</span><span style=\"color:#c0c5ce;\">&quot;];\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> states_parser = states\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">into_iter</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">map</span><span style=\"color:#c0c5ce;\">(LiteralParser::from)\n</span><span style=\"color:#c0c5ce;\">    .collect::&lt;Vec&lt;_&gt;&gt;();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create a parser that tries to match each state\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> states = IndexParser::new(states_parser);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// match a state, followed by a comma and a space, 5 times, and a newline\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> _validator = states\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">then</span><span style=\"color:#c0c5ce;\">(LiteralParser::from(&quot;</span><span style=\"color:#a3be8c;\">, </span><span style=\"color:#c0c5ce;\">&quot;))\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">repeat</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">..=</span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">then</span><span style=\"color:#c0c5ce;\">(LiteralParser::from(&quot;</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#c0c5ce;\">&quot;));\n</span><span style=\"color:#c0c5ce;\">\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "If you don't care about the output of the parser, but you want the LLM to adhere to a specific structure, you can also use a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "RegexParser" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " to match a regular expression:" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// You can also use a regex to match the same pattern. However, you will not get a parsed result once the generator is finished.\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> validator = RegexParser::new(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#a3be8c;\">&quot;((Alaska|Delaware|Florida|Georgia|Hawaii), ){5}\\n</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "generating-text",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#generating-text",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Generating Text",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Once you have defined a parser, you can generate text that adheres to the constraints defined by the parser. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "stream_structured_text" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " function takes a prompt and a parser and returns a stream of text that adheres to the constraints defined by the parser along with the results once generation is finished. The following example shows how to generate text using the parser defined above:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> llm = Phi::start().await;\n</span><span style=\"color:#b48ead;\">let </span><span style=\"color:#c0c5ce;\">(structured, result) = llm\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">stream_structured_text</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">A state that starts with A</span><span style=\"color:#c0c5ce;\">&quot;, validator)\n</span><span style=\"color:#c0c5ce;\">    .await\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">split</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">structured.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">println!(&quot;</span><span style=\"color:#a3be8c;\">Result: </span><span style=\"color:#d08770;\">{:?}</span><span style=\"color:#c0c5ce;\">&quot;, result.await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">());\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Constrained generation in Kalosm enables the generation of text that follows specific grammatical rules or patterns. By leveraging the Kalosm library, developers can create sophisticated language models tailored to their application's requirements. The example code showcases how to use Kalosm to generate text with constraints, providing a foundation for building more advanced natural language generation systems."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 256usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "text-embeddings",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#text-embeddings",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Text Embeddings",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Embeddings are a numerical representation of the meaning of some data. Text embeddings represent something about the meaning of some text. Embeddings can be used either directly or as input to machine learning models. In this chapter, we will learn how to use Kalosm to create text embeddings and integrate them into a database. We will also learn how to search the embedding database for documents that are similar to a given query."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-an-embedding-model",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-an-embedding-model",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating an Embedding model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "First, we need to create an embedding model. An embedding model is a machine learning model that can be used to create embeddings. Kalosm provides a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Bert" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct that can be used to create an embedding model."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> bert = Bert::default();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-embeddings",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-embeddings",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating Embeddings",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Once we have created an embedding model, we can use it to create embeddings. Kalosm provides a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Bert" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " struct that can be used to create embeddings." }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> text = &quot;</span><span style=\"color:#a3be8c;\">Hello, world!</span><span style=\"color:#c0c5ce;\">&quot;;\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> embeddings = bert.</span><span style=\"color:#96b5b4;\">embed</span><span style=\"color:#c0c5ce;\">(text).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">println!(&quot;</span><span style=\"color:#d08770;\">{:?}</span><span style=\"color:#c0c5ce;\">&quot;, embeddings);\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Try different values for the text we are embedding. How does the embedding change?"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-an-embedding-database",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-an-embedding-database",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating an Embedding Database",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Now that we know how to create embeddings, we can use them to create an embedding database. An embedding database is a data structure that stores embeddings and allows you to search for documents that are similar to a given query. Kalosm provides a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "DocumentTable" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct that can be used to create an embedding database linked to a table in a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://surrealdb.com/",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Surrealdb" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " database. You can choose a chunk strategy to use when creating the embedding database. A chunk strategy determines how documents are split into chunks before being embedded. In this example, we will use the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Sentence" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " chunk strategy, which splits documents into sentences before embedding them. The bert embedding model tends to work best with single sentence chunks."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::{language::*, *};\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create database connection\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> db = surrealdb::Surreal::new::&lt;surrealdb::engine::local::RocksDb&gt;(&quot;</span><span style=\"color:#a3be8c;\">./db/temp.db</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    .await\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Select a specific namespace / database\n</span><span style=\"color:#c0c5ce;\">db.</span><span style=\"color:#96b5b4;\">use_ns</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">test</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">use_db</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">test</span><span style=\"color:#c0c5ce;\">&quot;).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create a document table\n</span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> document_table = db\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">document_table_builder</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">documents</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Store the embedding database in the ./db/embeddings.db file\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">at</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">./db/embeddings.db</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "adding-documents-to-the-embedding-database",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#adding-documents-to-the-embedding-database",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Adding Documents to the Embedding Database",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Once you have created an embedding database, you can add documents to it with the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "extend" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method. The  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "extend" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method takes something that can be turned into documents and adds them to the embedding database. In this example, we will add documents from a RSS feed to the embedding database."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> nyt = RssFeed::new(\n</span><span style=\"color:#c0c5ce;\">    Url::parse(&quot;</span><span style=\"color:#a3be8c;\">https://rss.nytimes.com/services/xml/rss/nyt/US.xml</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">(),\n</span><span style=\"color:#c0c5ce;\">);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Fetch the documents from the feed\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> documents = nyt.</span><span style=\"color:#96b5b4;\">into_documents</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#65737e;\">// And insert them into the database\n</span><span style=\"color:#b48ead;\">for</span><span style=\"color:#c0c5ce;\"> document in documents {\n</span><span style=\"color:#c0c5ce;\">    document_table.</span><span style=\"color:#96b5b4;\">insert</span><span style=\"color:#c0c5ce;\">(document).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::p::TAG_NAME,
                                namespace: dioxus_elements::p::NAME_SPACE,
                                attrs: &[],
                                children: &[
                                    dioxus_core::TemplateNode::Text {
                                        text:
                                            "This example uses rss context, but you can also use ",
                                    },
                                    {
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/live_qa.rs",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "audio" }],
                            }
                                    },
                                    dioxus_core::TemplateNode::Text { text: ", " },
                                    {
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/blob/94542cf49923cb4e15e34244336f8844ee2194c4/interfaces/kalosm/examples/fs_context.rs#L31",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "filesystem" }],
                            }
                                    },
                                    dioxus_core::TemplateNode::Text { text: ", or " },
                                    {
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/blob/94542cf49923cb4e15e34244336f8844ee2194c4/interfaces/kalosm-language/src/context/search/mod.rs#L16-L31",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "search engine" }],
                            }
                                    },
                                    dioxus_core::TemplateNode::Text {
                                        text: " contextYou can also use a ",
                                    },
                                    {
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/blob/94542cf49923cb4e15e34244336f8844ee2194c4/interfaces/kalosm/examples/fs_context.rs#L19-L20",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "fuzzy search engine" }],
                            }
                                    },
                                    dioxus_core::TemplateNode::Text {
                                        text: " with the same api if you prefer traditional search",
                                    },
                                ],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "searching-the-embedding-database",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#searching-the-embedding-database",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Searching the Embedding Database",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Next, you can use search through the documents you embedded with the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "search" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method. The  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "search" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method takes a query and returns a list of documents that are similar to the query. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "search" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " method also takes a  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "limit" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " parameter that determines how many documents to return."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">loop </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> user_question = </span><span style=\"color:#96b5b4;\">prompt_input</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">Query: </span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> user_question_embedding = document_table\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">embedding_model_mut</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">embed</span><span style=\"color:#c0c5ce;\">(&amp;user_question)\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    println!(\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">vector: </span><span style=\"color:#d08770;\">{:?}</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">        document_table\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">select_nearest_embedding</span><span style=\"color:#c0c5ce;\">(user_question_embedding, </span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">            .await\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    );\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "In this chapter, we learned how to use Kalosm to create embeddings and integrate them into a database. We also learned how to search the embedding database for similar documents."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 288usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "kalosm-guides",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#kalosm-guides",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Kalosm Guides",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This section contains guides that go into more detail about specific Kalosm features to a build more complex application."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[{
                                    dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "guides/retrieval_augmented_generation",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "Create a chatbot that understands your surroundings with retrieval-augmented generation"
                                }],
                            }
                                }],
                            }
                        }],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 320usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "retrieval-augmented-generation-in-kalosm",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#retrieval-augmented-generation-in-kalosm",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Retrieval-augmented generation in Kalosm",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "introduction",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#introduction",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Introduction",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Retrieval-augmented generation in Kalosm is a powerful approach that combines natural language generation with real-time information from audio data. This guide will walk you through the process of implementing retrieval-augmented generation using the Kalosm library."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: p :: TAG_NAME,
                        namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Before you begin, make sure you have the Kalosm library installed and set up in your Rust project. You can refer to the "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value : "..",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "Introduction" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        { text : " for instructions on how to install Kalosm." }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-a-transcription-model",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-a-transcription-model",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating a Transcription Model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "First we need to create a transcription model. Kalosm provides a  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Whisper" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct that serves as a transcription model. You can initialize it as follows:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::audio::*;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::*;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">std::sync::Arc;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">tokio::time::{Duration, Instant};\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = WhisperBuilder::default()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_source</span><span style=\"color:#c0c5ce;\">(WhisperSource::MediumEn)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()?;\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This sets up a transcription model using the Whisper source for English ( "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "WhisperSource::MediumEn" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "). Adjust the source according to your language preferences."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "creating-a-context-database",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#creating-a-context-database",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Creating a Context Database",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "The next step is to create a context database. Kalosm provides  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "DocumentTable" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " struct that indexes a " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://surrealdb.com/",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Surrealdb" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " database table with a vector database which can serve as our context database. We need to wrap the database in a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Arc" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " so that it can be shared across threads:" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#65737e;\">// Create database connection\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> db = surrealdb::Surreal::new::&lt;surrealdb::engine::local::RocksDb&gt;(&quot;</span><span style=\"color:#a3be8c;\">./db/temp.db</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    .await\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Select a specific namespace / database\n</span><span style=\"color:#c0c5ce;\">db.</span><span style=\"color:#96b5b4;\">use_ns</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">test</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">use_db</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">test</span><span style=\"color:#c0c5ce;\">&quot;).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create a new document database table\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> document_table = Arc::new(\n</span><span style=\"color:#c0c5ce;\">    db.</span><span style=\"color:#96b5b4;\">document_table_builder</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">documents</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">        </span><span style=\"color:#65737e;\">// Store the embedding database at ./db/embeddings.db\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">at</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">./db/embeddings.db</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">(),\n</span><span style=\"color:#c0c5ce;\">);\n</span><span style=\"color:#c0c5ce;\">\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "recording-audio",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#recording-audio",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Recording Audio",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Next, we need to record audio data. Kalosm provides a  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "MicInput",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " struct that can be used to record audio data.",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    std::thread::spawn(</span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">|| {\n</span><span style=\"color:#c0c5ce;\">        tokio::runtime::Runtime::new()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">block_on</span><span style=\"color:#c0c5ce;\">(async </span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">                </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> recording_time = Duration::from_secs(</span><span style=\"color:#d08770;\">30</span><span style=\"color:#c0c5ce;\">);\n</span><span style=\"color:#c0c5ce;\">                </span><span style=\"color:#b48ead;\">loop </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">                    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> _input = MicInput::default()\n</span><span style=\"color:#c0c5ce;\">                        .</span><span style=\"color:#96b5b4;\">record_until</span><span style=\"color:#c0c5ce;\">(Instant::now() + recording_time)\n</span><span style=\"color:#c0c5ce;\">                        .await\n</span><span style=\"color:#c0c5ce;\">                        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">                }\n</span><span style=\"color:#c0c5ce;\">            })\n</span><span style=\"color:#c0c5ce;\">    });\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This code records audio until a certain point in time, providing a continuous stream of audio data."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "transcribing-audio",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#transcribing-audio",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Transcribing Audio",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Once you have recorded audio data, you can transcribe it into text using the transcription model. The  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "transcribe" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " method returns a stream of text snippets along with the confidence of the transcription. We can add the text snippets to the context database to create a real-time context."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> document_table = document_table.</span><span style=\"color:#96b5b4;\">clone</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    std::thread::spawn(</span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">|| {\n</span><span style=\"color:#c0c5ce;\">        tokio::runtime::Runtime::new()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">block_on</span><span style=\"color:#c0c5ce;\">(async </span><span style=\"color:#b48ead;\">move </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">                </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> recording_time = Duration::from_secs(</span><span style=\"color:#d08770;\">30</span><span style=\"color:#c0c5ce;\">);\n</span><span style=\"color:#c0c5ce;\">                </span><span style=\"color:#b48ead;\">loop </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">                    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> input = MicInput::default()\n</span><span style=\"color:#c0c5ce;\">                        .</span><span style=\"color:#96b5b4;\">record_until</span><span style=\"color:#c0c5ce;\">(Instant::now() + recording_time)\n</span><span style=\"color:#c0c5ce;\">                        .await\n</span><span style=\"color:#c0c5ce;\">                        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">                    </span><span style=\"color:#b48ead;\">if let </span><span style=\"color:#c0c5ce;\">Ok(</span><span style=\"color:#b48ead;\">mut</span><span style=\"color:#c0c5ce;\"> transcribed) = model.</span><span style=\"color:#96b5b4;\">transcribe</span><span style=\"color:#c0c5ce;\">(input) {\n</span><span style=\"color:#c0c5ce;\">                        </span><span style=\"color:#b48ead;\">while let </span><span style=\"color:#c0c5ce;\">Some(transcribed) = transcribed.</span><span style=\"color:#96b5b4;\">next</span><span style=\"color:#c0c5ce;\">().await {\n</span><span style=\"color:#c0c5ce;\">                            </span><span style=\"color:#b48ead;\">if</span><span style=\"color:#c0c5ce;\"> transcribed.</span><span style=\"color:#96b5b4;\">probability_of_no_speech</span><span style=\"color:#c0c5ce;\">() &lt; </span><span style=\"color:#d08770;\">0.90 </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">                                </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> document =\n</span><span style=\"color:#c0c5ce;\">                                    transcribed.</span><span style=\"color:#96b5b4;\">text</span><span style=\"color:#c0c5ce;\">().</span><span style=\"color:#96b5b4;\">into_document</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">                                document_table.</span><span style=\"color:#96b5b4;\">insert</span><span style=\"color:#c0c5ce;\">(document).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">                            }\n</span><span style=\"color:#c0c5ce;\">                        }\n</span><span style=\"color:#c0c5ce;\">                    }\n</span><span style=\"color:#c0c5ce;\">                }\n</span><span style=\"color:#c0c5ce;\">            })\n</span><span style=\"color:#c0c5ce;\">    });\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "create-chat-model",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#create-chat-model",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Create Chat Model",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Next, we need to create a chat model. We can use the default chat model provided by Kalosm along with the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Chat" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " interface." }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> model = Llama::new_chat();\n</span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> chat = Chat::builder(&amp;</span><span style=\"color:#b48ead;\">mut</span><span style=\"color:#c0c5ce;\"> model).</span><span style=\"color:#96b5b4;\">with_system_prompt</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "implementing-retrieval-augmented-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#implementing-retrieval-augmented-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Implementing Retrieval-Augmented Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Finally, we can implement the main chat loop that asks the user for input, searches the context database for relevant information, and generates a response using the chat model."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">loop </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Ask the user for a question\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> user_question = </span><span style=\"color:#96b5b4;\">prompt_input</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">&gt; </span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Search for relevant context in the document engine\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> context = document_table\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">select_nearest</span><span style=\"color:#c0c5ce;\">(&amp;user_question, </span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">        .await?\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">into_iter</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">map</span><span style=\"color:#c0c5ce;\">(|</span><span style=\"color:#bf616a;\">document</span><span style=\"color:#c0c5ce;\">| {\n</span><span style=\"color:#c0c5ce;\">            format!(\n</span><span style=\"color:#c0c5ce;\">                &quot;</span><span style=\"color:#a3be8c;\">Title: </span><span style=\"color:#d08770;\">{}</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">Body: </span><span style=\"color:#d08770;\">{}</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">                document.record.</span><span style=\"color:#96b5b4;\">title</span><span style=\"color:#c0c5ce;\">(),\n</span><span style=\"color:#c0c5ce;\">                document.record.</span><span style=\"color:#96b5b4;\">body</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            )\n</span><span style=\"color:#c0c5ce;\">        })\n</span><span style=\"color:#c0c5ce;\">        .collect::&lt;Vec&lt;_&gt;&gt;()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">join</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#c0c5ce;\">&quot;);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Format a prompt with the question and context\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> prompt = format!(\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">Here is the relevant context:</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#d08770;\">{context}</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">Given that context, answer the following question:</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#d08770;\">{user_question}</span><span style=\"color:#c0c5ce;\">&quot;\n</span><span style=\"color:#c0c5ce;\">    );\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Display the prompt to the user for debugging purposes\n</span><span style=\"color:#c0c5ce;\">    println!(&quot;</span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, prompt);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// And finally, respond to the user\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> output_stream = chat.</span><span style=\"color:#96b5b4;\">add_message</span><span style=\"color:#c0c5ce;\">(prompt).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    print!(&quot;</span><span style=\"color:#a3be8c;\">Bot: </span><span style=\"color:#c0c5ce;\">&quot;);\n</span><span style=\"color:#c0c5ce;\">    output_stream.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Retrieval-augmented generation in Kalosm makes it possible for language models to generate responses based on up-to-date information from the real world. This guide has shown you how to implement retrieval-augmented generation using the Kalosm library. For more information, the reference documentation documents more details about: "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "../reference/transcription",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Whisper" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ", " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "../reference/llms/context",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "DocumentTable" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ", and " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "../reference/llms/chat",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Chat" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : "." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 0usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "introduction",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#introduction",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Introduction",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum is a graph editor for AI workflows with a focus on community made plugins, local AI and safety."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 0usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Demo Screenshot",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "installation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#installation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Installation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Before you get started using Floneum, make sure to ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value:
                                            "https://github.com/floneum/floneum/releases/tag/v0.2.0",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "download it from the releases page",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: "." },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "features",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#features",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Features" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Visual interface: You can use Floneum without any knowledge of programming. The visual graph editor makes it easy to combine community made plugins with local AI models"
                        }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Instantly run local large language models: Floneum does not require any external dependencies or even a GPU to run. It uses "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/rustformers/llm",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "llm" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " to run large language models locally. Because of this, you can run Floneum with your data without worrying about privacy"
                        }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Plugins: By combining large language models with plugins, you can improve their performance and make models work better for your specific use case. All plugins run in an isolated environment so you don't need to trust any plugins you load. Plugins can only interact with their environment in a safe way"
                        }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Multi-language plugins: Plugins can use in any language that supports web assembly. In addition to the API that can be accessed in any language, Floneum has a rust wrapper with ergonomic macros that make it simple to create plugins"
                        }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Controlled text generation: Plugins can control the output of the large language models with a process similar to jsonformer or guidance. This allows plugins to force models to output valid json, or any other structure they define. This can be useful when communicating between a language model and a typed API"
                        }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "guide",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#guide",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Guide" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: p :: TAG_NAME,
                                namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "If you are looking to use Floneum, you can read the "
                                },
                                {
                                    dioxus_core :: TemplateNode :: Element
                                    {
                                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                        [dioxus_core :: TemplateAttribute :: Static
                                        {
                                            name : dioxus_elements :: a :: href.0, namespace :
                                            dioxus_elements :: a :: href.1, value : "./user",
                                        }], children : &
                                        [dioxus_core :: TemplateNode :: Text
                                        { text : "User Documentation" }],
                                    }
                                }, dioxus_core :: TemplateNode :: Text { text : "." }],
                            }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: p :: TAG_NAME,
                                namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "If you are looking to develop plugins for Floneum, you can read the "
                                },
                                {
                                    dioxus_core :: TemplateNode :: Element
                                    {
                                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                        [dioxus_core :: TemplateAttribute :: Static
                                        {
                                            name : dioxus_elements :: a :: href.0, namespace :
                                            dioxus_elements :: a :: href.1, value : "./developer",
                                        }], children : &
                                        [dioxus_core :: TemplateNode :: Text
                                        { text : "Developer Documentation" }],
                                    }
                                }],
                            }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: p :: TAG_NAME,
                                namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "If you are interested in contributing to Floneum, you can read the "
                                },
                                {
                                    dioxus_core :: TemplateNode :: Element
                                    {
                                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                        [dioxus_core :: TemplateAttribute :: Static
                                        {
                                            name : dioxus_elements :: a :: href.0, namespace :
                                            dioxus_elements :: a :: href.1, value : "./contributing",
                                        }], children : &
                                        [dioxus_core :: TemplateNode :: Text
                                        { text : "Contributing Documentation" }],
                                    }
                                }],
                            }
                                    }],
                                }
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[&[2u8, 0u8]],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(
                None,
                ___TEMPLATE,
                Box::new([]),
                Box::new([Box::new([{
                    dioxus_core :: Attribute ::
            new(dioxus_elements :: img :: src.0, manganis :: mg!
            (file("https://github.com/Demonthos/floneum/assets/66571940/c60d621d-72b9-423c-b1d5-57cdb737e449")),
            dioxus_elements :: img :: src.1, dioxus_elements :: img :: src.2)
                }])]),
            );
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 32usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "floneum",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#floneum",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Floneum" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum is a graph editor for AI workflows. It allows you to build workflows using either built in or "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "user/./community_plugins",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "community" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " plugins." }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This guide is focused on users of Floneum, if you are interested in expanding Floneum's capibilities with a plugin, the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "user/../developer",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "developer guide" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " has more information." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 64usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "your-first-workflow",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#your-first-workflow",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Your First Workflow",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "In this example, we will create a workflow that answers questions from a database of information."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::p::TAG_NAME,
                                namespace: dioxus_elements::p::NAME_SPACE,
                                attrs: &[],
                                children: &[
                                    dioxus_core::TemplateNode::Text {
                                        text: "Before starting this guide, you must ",
                                    },
                                    {
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/releases/tag/v0.2.0",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "download" }],
                            }
                                    },
                                    dioxus_core::TemplateNode::Text { text: " Floneum." },
                                ],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "text-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#text-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Text Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "First, let's use a " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./concepts/models",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Large Language Model" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " to generate text from a prompt. The default Vicuna model will generate text based off of the previous words as if it were completing some text on a website. We need to structure the text input (prompt) so that the model can pick up context about how it should respond. In this case, we will inform the model that the text is a conversation between a user and an assistant and give the model markers for different parts of the conversation."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "To start working with language models, we need to create a prompt. We will split out two different sections of our prompt: the  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "context" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " and the  " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "question" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ". We can combine these two sections with a format node which will fill in any  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "{}" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "'s in the prompt with the strings passed in in order."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "To add a format node, right click and select the  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: "format" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " node from the drop down:",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 0usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Dropdown with generate text node selected",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Next enter the text that the language model should complete:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">A conversation between a user and an assistant. The assistant will accept context about the world with </span><span style=\"color:#d08770;\">true</span><span style=\"color:#c0c5ce;\">, up to date information about the world. The assistant uses the infomation in the context to answer susinctly:\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### Context\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">Clouds are green.\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### User\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">What color are clouds?\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### Assistant\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">green\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### Context\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">Floeum is a graph </span><span style=\"color:#d08770;\">AI</span><span style=\"color:#c0c5ce;\"> editor\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### User\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">What is the best graph </span><span style=\"color:#d08770;\">AI</span><span style=\"color:#c0c5ce;\"> editor\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### Assistant\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">Floeum\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### Context\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">{}\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### User\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">{}\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">### Assistant\n</span><span style=\"color:#c0c5ce;\">\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Then let's create two more format nodes, one for the question and one for the context that is needed to answer the question:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 1usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Floneum with question and context nodes",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Next, we can generate text from a model with the generate text node:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 2usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Floneum with generate text node",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Finally, click run to start the model running. This will take several minutes the first time you run the node because the node needs to download the model. Futures runs will be significantly faster."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "embeddings",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#embeddings",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Embeddings" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "You may have noticed that the lanuage model outputted the wrong answer. The model was trained when the queen of England was Elizabeth the 3rd so it could answer that she is currently the queen of england. To fix this issue, we will use a database of documents"
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "To overcome this limitation, we need to use a list of more up to date information and provide information from that list to the model."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "This list of facts will be stored in a " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "./concepts/embedding_db",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Embedding Database" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ". An embedding is a representation of the meaning of some text. A Database is just a structured way to store data. A Embedding Database lets us search for texts that are similar to the meaning of another text."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "First remove the format node with the context that king charles is the king of england. We will replace this with a more flexable embedding database."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Then, let's create a database. Add the  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "embedding_db",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: " node:" },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 3usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Floneum with embedding db node",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Next, set the seperator to  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: "," }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " and the documents to index (add to the database) to:",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">The king of england is Charles </span><span style=\"color:#d08770;\">III</span><span style=\"color:#c0c5ce;\">,The Pacific Ocean is the largest ocean which covers one-third of the Earth\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Finnally, run the node to create the database.",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "search",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#search",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Search" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Now, let's search the database for documents close to our query instead of relying on the prewritten context."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Then add an embedding node which converst some text from the query into the embedded meaning in the lanuage model selected:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 4usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Floneum with query embedding node",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Finally, add a search node that searches for the top 1 closest documents to the query and adds that context to the prompt:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 5usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Floneum with search node",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "conclusion",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#conclusion",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Conclusion" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Now you are ready to create workflows of your own. For more information about the concepts used in this guide, see the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./concepts",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "concepts" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " chapter. If you aren interested in creating your own plugin, see the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "../developer",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "developer" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : " guide." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[
                &[7u8, 0u8],
                &[11u8, 0u8],
                &[13u8, 0u8],
                &[21u8, 0u8],
                &[28u8, 0u8],
                &[30u8, 0u8],
            ],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(
                None,
                ___TEMPLATE,
                Box::new([]),
                Box::new([
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/first_workflow_add_node.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/first_workflow_1.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/first_workflow_2.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/first_workflow_3.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/first_workflow_4.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/first_workflow_5.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                ]),
            );
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 96usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "important-concepts",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#important-concepts",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Important Concepts",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::a::TAG_NAME,
                                            namespace: dioxus_elements::a::NAME_SPACE,
                                            attrs: &[dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::a::href.0,
                                                namespace: dioxus_elements::a::href.1,
                                                value: "concepts/./models",
                                            }],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "Language Models",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::a::TAG_NAME,
                                            namespace: dioxus_elements::a::NAME_SPACE,
                                            attrs: &[dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::a::href.0,
                                                namespace: dioxus_elements::a::href.1,
                                                value: "concepts/./embedding",
                                            }],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "Embedding",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::a::TAG_NAME,
                                            namespace: dioxus_elements::a::NAME_SPACE,
                                            attrs: &[dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::a::href.0,
                                                namespace: dioxus_elements::a::href.1,
                                                value: "concepts/./embedding_db",
                                            }],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "Embedding Databases",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::a::TAG_NAME,
                                            namespace: dioxus_elements::a::NAME_SPACE,
                                            attrs: &[dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::a::href.0,
                                                namespace: dioxus_elements::a::href.1,
                                                value: "concepts/./structured",
                                            }],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "Strutured Generation",
                                            }],
                                        }
                                    }],
                                }
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 128usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "language-models",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#language-models",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Language Models",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Language models are powerful tools that help predict what words come next in a sentence. However, they have some limitations. They can only consider a limited number of words in the past when making predictions, which can make them shortsighted. They also struggle with planning and logical reasoning."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "To overcome these limitations, language models can be enhanced with plugins. These plugins can "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./embedding_db",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "improve the model's memory" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ", allowing it to remember more information from previous words. They can also provide additional context by integrating search results into the model's predictions. By using these plugins, language models become more effective and capable of generating more accurate and meaningful text."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 160usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "structured-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#structured-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Structured Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Structured generation is a way to control what a language model can generate. While models have the ability to generate any text, structured generation lets you set specific boundaries on the output. For instance, you can restrict the model to generate only numbers between 1 and 5, or create a list of specific strings. This helps ensure that the generated content aligns with your desired constraints or requirements."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 192usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "embeddings",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#embeddings",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Embeddings" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Embeddings are a representation of the meaning of some text. They can be used to compare the meaning of two different texts or search for documents with a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./embedding_db",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Embedding Database" }],
                    }
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Embeddings are a way to understand the meaning of text. They provide a representation of the meaning of the words used. It lets us focus on the meaning of the text instead of the specific wording of the text."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "They can be used to compare the meaning of two different texts or search for documents with a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./embedding_db",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Embedding Database" }],
                    }
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 224usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "embedding-database",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#embedding-database",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Embedding Database",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [{
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value : "./embedding",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Embedding" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " Databases are powerful tools that enable you to search for texts that have similar meanings to a given text. They work by representing the meaning of each text as a mathematical vector, or embedding, in a high-dimensional space. This allows the database to efficiently compare and match texts based on their semantic similarity, rather than relying solely on exact word matches. By using embedding databases, you can find related texts and discover relevant information even if the wording is different."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Combined with a model, it can be used to extend the memory of the model or allow the model to search through a larger database of information."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 256usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "community-plugins",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#community-plugins",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Community Plugins",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Community Plugins are additional features that you can add to Floneum. They allow you to expand the capabilities of Floneum without compromising the security of your computer."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "To load a community plugin, you simply download a .wasm file and select the file path in the load plugin textbox in the top left of the application. Floneum ensures that the plugin operates separately from your computer, with restricted access to the network and filesystem."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "You can load community plugins by downloading a  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: ".wasm" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " file and then loading the plugin from the file path:",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 0usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Plugin Textbox",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "By using community plugins, you can enhance Floneum's functionality in a safe and controlled manner."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[&[4u8, 0u8]],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(
                None,
                ___TEMPLATE,
                Box::new([]),
                Box::new([Box::new([{
                    dioxus_core::Attribute::new(
                        dioxus_elements::img::src.0,
                        manganis::mg!(file("./public/assets/load_local_plugin.png")),
                        dioxus_elements::img::src.1,
                        dioxus_elements::img::src.2,
                    )
                }])]),
            );
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 288usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "plugins",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#plugins",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Plugins" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "You can extend Floneum using ",
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://webassembly.org/getting-started/developers-guide/",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Web Assembly" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text { text: "." },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Plugins can be created in any language that supports WASM. Currently, only Rust has a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/tree/main/rust_adapter",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "launage-specific wrapper" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ". If you would like to use another language, you can generate bindings using a tool like "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/bytecodealliance/wit-bindgen",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "wit-bindgen" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " with the " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/WebAssembly/component-model",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "WASM interface types" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " defined in the " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/blob/main/wit/plugin.wit",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "plugin.wit" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : " file." }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 320usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "your-first-plugin",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#your-first-plugin",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Your First plugin",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "This example will use ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://www.rust-lang.org/",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "rust" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " to build an new plugin for Floneum.",
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "First, edit your cargo.toml to add floneum_rust and wit-bindgen as dependencies and change the crate type to a dynamically linked (C-like) library"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">[lib]\n</span><span style=\"color:#b48ead;\">crate</span><span style=\"color:#c0c5ce;\">-</span><span style=\"color:#b48ead;\">type </span><span style=\"color:#c0c5ce;\">= [&quot;</span><span style=\"color:#a3be8c;\">cdylib</span><span style=\"color:#c0c5ce;\">&quot;]\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">[dependencies]\n</span><span style=\"color:#c0c5ce;\">floneum_rust = { git = &quot;</span><span style=\"color:#a3be8c;\">https://github.com/floneum/floneum</span><span style=\"color:#c0c5ce;\">&quot; }\n</span><span style=\"color:#c0c5ce;\">wit-bindgen = { git = &quot;</span><span style=\"color:#a3be8c;\">https://github.com/bytecodealliance/wit-bindgen</span><span style=\"color:#c0c5ce;\">&quot;, rev = &quot;</span><span style=\"color:#a3be8c;\">285f0c6ad5da3d6cd8ef2e0635df51f229d6578f</span><span style=\"color:#c0c5ce;\">&quot; }\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Then create your plugin with the export plugin macro:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">floneum_rust::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">export_plugin</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#65737e;\">/// adds two numbers\n</span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">add</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#bf616a;\">first</span><span style=\"color:#c0c5ce;\">: </span><span style=\"color:#b48ead;\">i64</span><span style=\"color:#c0c5ce;\">, </span><span style=\"color:#bf616a;\">second</span><span style=\"color:#c0c5ce;\">: </span><span style=\"color:#b48ead;\">i64</span><span style=\"color:#c0c5ce;\">) -&gt; </span><span style=\"color:#b48ead;\">i64 </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    first + second\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Next, build your plugin:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">cargo install --git https:</span><span style=\"color:#65737e;\">//github.com/floneum/floneum floneum-cli\n</span><span style=\"color:#c0c5ce;\">floneum build --release\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::p::TAG_NAME,
                                namespace: dioxus_elements::p::NAME_SPACE,
                                attrs: &[],
                                children: &[
                                    dioxus_core::TemplateNode::Text {
                                        text: "You can look at the default plugins ",
                                    },
                                    {
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/tree/main/floneum/plugins",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "here" }],
                            }
                                    },
                                    dioxus_core::TemplateNode::Text {
                                        text: " to see how more complex plugins work",
                                    },
                                ],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Finally, load your plugin by running the main Floneum application and typing the path to your  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : ".wasm" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text : " file in the load plugin text box in the top left."
                }],
            }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 352usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "contributing",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#contributing",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Contributing",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "To start contributing to Floneum, we provide a list of ",
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Good first issues" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text { text: "." },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "If you are interesting in contributing to Floneum, consider reaching out on "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://discord.gg/dQdmhuB8q5",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "discord" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ". We are always happy to help new contributors get started!"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "project-architecture",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#project-architecture",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Project Architecture",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text { text: "Plugins:" }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://webassembly.org/",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "WASM" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: ":" },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Wasm is an assembly language originally built for the web. It has a few characteristics that make it appealing for Floneum plugins:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Many languages can compile to it",
                                    }],
                                }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "When you run WASM, it is sandboxed from you environment by default. You have to explicitly allow WASM plugins to access certain resources"
                        }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://github.com/bytecodealliance/wasmtime",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Wasmtime",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: ":" },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Wasmtime is a library Floneum uses to run WASM plugins",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://wasi.dev/",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "WASI" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: ":" },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "WASI is a set of interfaces for WASM programs. It is a set of resources you can allow your WASM plugin to access. Specifically, it allows controlled, file system, network, and time access. You can think of WASI as something like the \"standard library\" for WASM programs."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "WIT (wasm component model)" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text { text: ":" },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "The WASM component model allows the environment that runs WASM (Floneum) to declare a typed interface that Plugins (Nodes) can use. Because the interface is typed, each language can read the common environment declaration and create wrappers that work well with that language."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "In Floneum, the interface is declared in the ",
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/blob/main/wit/plugin.wit",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "plugin.wit" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text { text: " file." },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This file is used to create the language specific types in each plugin using "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/bytecodealliance/wit-bindgen",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "wit-bindgen" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ". We also provide an extra level of wrapping with a rust macro and some special functions specific to rust in the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/tree/main/rust_adapter",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "rust adapter" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : " package" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "It is also used to declare the interface in wasmtime ",
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/blob/cd83ac7d3487826c54789619529db53125859923/plugin/src/lib.rs#L218",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "here" }],
                    }
                            },
                            dioxus_core::TemplateNode::Text { text: ". " },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: p :: TAG_NAME,
                        namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Limitations:Currently, the Floneum bindings uses a lot of *-id types. These types represent resources that live in Floneum like a model instance. We also use ids to represent recursive types like the structure type used to constrain structured generation.As WIT matures, first party "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "resource" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " types will be implemented that make these id types unnecessary"
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text { text: "UI:" }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "The main UI for Floneum is written in ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://lib.rs/crates/dioxus",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "Diouxs" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: ". The code for the UI of floneum can be found in the ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://github.com/floneum/floneum/tree/main/src",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "main package",
                                    }],
                                }
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 0usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "posts",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#posts",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "Posts" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::a::href.0,
                                    namespace: dioxus_elements::a::href.1,
                                    value: "./structured_generation_visualized",
                                }],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Structured Generation Visualized",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::a::href.0,
                                    namespace: dioxus_elements::a::href.1,
                                    value: "./kalosm_0_2",
                                }],
                                children: &[dioxus_core::TemplateNode::Text { text: "Kalosm 0.2" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::a::href.0,
                                    namespace: dioxus_elements::a::href.1,
                                    value: "./floneum_0_2",
                                }],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Floneum 0.2",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[dioxus_core::TemplateAttribute::Static {
                                    name: dioxus_elements::a::href.0,
                                    namespace: dioxus_elements::a::href.1,
                                    value: "./announcing_floneum",
                                }],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Announcing Floneum",
                                }],
                            }
                        }],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 32usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "announcing-floneum-a-graph-editor-for-local-ai-workflows",
                        }],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "#announcing-floneum-a-graph-editor-for-local-ai-workflows",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: class.0, namespace :
                            dioxus_elements :: a :: class.1, value : "header",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Announcing Floneum: A Graph Editor for Local AI Workflows"
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "We are thrilled to introduce Floneum, an intuitive graph editor designed specifically for local AI workflows. Floneum empowers users to effortlessly guide large language models and build structured workflows tailored to their specific use cases."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 0usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Floneum Demo",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "what-is-floneum",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#what-is-floneum",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "What is Floneum?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum is a user-friendly editor for visual AI workflows. Unlike existing tools that may have a high barrier to entry or allow limited control, Floneum provides a solution that is both easy to use and allows for greater customization."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "For instance, while the chat GPT interface provides a straightforward entry point, it quickly becomes challenging to create structured workflows. Imagine wanting to search through files to find specific ones, such as all .txt files related to travel, and then upload them. With Floneum, you can achieve this seamlessly within a structured workflow, eliminating the need for manual interaction with external tools."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "On the other end of the spectrum, tools like " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/hwchase17/langchain",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Langchain" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " offer extensive workflow customization but come with more system requirements and potential security concerns. Langchain requires users to install tools like Python and CUDA, making it less accessible to non-developers. In addition to this, building workflows in Python code can be impractical for individuals without programming expertise. Finally, plugins in Langchain are not sandboxed, which can expose users to malware or security risks when incorporating third-party libraries."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum is a single executable that runs models locally, eliminating the need for complex installations. The heart of Floneum is its graph-based editor, designed to enable users without programming knowledge to build and manage their AI workflows seamlessly."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "what-can-you-build-with-floneum",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#what-can-you-build-with-floneum",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "What Can You Build with Floneum?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "While Floneum is still in the early stages of development, it already offers a range of built-in plugins that empower users to achieve their goals effectively. As of writing this post, the following plugins are available within Floneum:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "embedding",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "add_embedding",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "embedding_db",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "format",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "generate_text",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "generate_structured_text",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "search",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "search_engine",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "if_statement",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "contains",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "write_to_file",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "read_from_file",
                                            }],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::code::TAG_NAME,
                                            namespace: dioxus_elements::code::NAME_SPACE,
                                            attrs: &[],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "python",
                                            }],
                                        }
                                    }],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "These plugins cover various functionalities, such as embedding data, generating text, searching through resources, and more. However, it's important to note that the capabilities of Floneum extend beyond these built-in plugins."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum is designed to support an expanding ecosystem of plugins. In the future, additional plugins will be added to enhance its functionality further. Furthermore, if the built-in plugins don't precisely fit your application, Floneum allows you to extend its capabilities with plugins that are fully sandboxed within their own environment. Through the utilization of a WebAssembly (WASM) compiler, plugins can only access resources within their designated sandbox. This ensures that you can trust Floneum to prevent any malicious activity from impacting your computer."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "whats-next-for-floneum",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#whats-next-for-floneum",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "What's Next for Floneum?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "We are excited about the future of Floneum and the upcoming features and improvements we have planned. Here are some of the things we plan to work on:"
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "API based model integrations: We will be integrating with popular hosted AI models, including Chat GPT and more, to allow users to seamlessly incorporate these models into their Floneum workflows."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Improved Plugin System: We would like to continuously improve the plugin system as Floneum develops. Some of the improvements we are looking at includes introducing plugins as values, enabling more advanced control flow, and developing tutorials for writing plugins in additional languages."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Package Manager: In the future, we would like to introduce a package manager to simplify the process of discovering, installing, and managing plugins. This will enable users to easily extend Floneum's functionality and explore the ecosystem of community-contributed plugins."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Support for Other Model Types: In addition to language models, we have plans to expand Floneum's support to other model types, such as image generation, classification, and more. This will broaden the range of AI applications that can be built using Floneum."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "We look forward to sharing these exciting updates with you as we continue to evolve Floneum. Stay tuned for more information and be part of the Floneum community as we shape the future of local AI workflows."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "getting-started",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#getting-started",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Getting Started",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "To get started using Floneum, you can go to the " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://floneum.com/docs/user/",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "user documentation" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ". If you are interested in developing plugins for Floneum, you start with the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://floneum.com/docs/developer/",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "developer documentation" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : "." }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "Finally, if you are interested in Floneum, ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://discord.gg/dQdmhuB8q5",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "join our discord community",
                                    }],
                                }
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[&[2u8, 0u8]],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(
                None,
                ___TEMPLATE,
                Box::new([]),
                Box::new([Box::new([{
                    dioxus_core::Attribute::new(
                        dioxus_elements::img::src.0,
                        manganis::mg!(file("./public/assets/demo-img.png")),
                        dioxus_elements::img::src.1,
                        dioxus_elements::img::src.2,
                    )
                }])]),
            );
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 64usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "announcing-floneum-02",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#announcing-floneum-02",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Announcing Floneum 0.2",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum 0.2 is here with improvements ranging from UI improvements and workflow sharing to web scraping and plugin distribution!"
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum is a visual editor for AI workflows packaged as an easy to install application with no external dependencies. It makes it easy to build workflows that use large language models like Llama 2. The best part is, Floneum is fully open source and local. You control your data and your workflows."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "what-is-new-in-02",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#what-is-new-in-02",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "What is new in 0.2?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "improved-web-scraping",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#improved-web-scraping",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Improved Web Scraping",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "When the MVP of Floneum was released, it only contained plugins for reading from wikipedia and getting the raw (unreadable) HTML from a website. In the 0.4 release, we added a new  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "Get Article" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " plugin that allows you to read an article from " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: em :: TAG_NAME,
                        namespace : dioxus_elements :: em :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "any" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " URL automatically!" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 0usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Get Article Plugin Demo",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "reading-website-feeds",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#reading-website-feeds",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Reading Website Feeds",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Many websites have a feed of recently published content in an RSS stream. You can now use that RSS feed within Floneum to automatically gather a series of articles. For example, you can use the RSS feed from the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://rss.nytimes.com/services/xml/rss/nyt/World.xml",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "new york times paper" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " to get the latest world news:" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 1usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "RSS Plugin Demo",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "automated-browsing-plugins",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#automated-browsing-plugins",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Automated Browsing Plugins",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Up until this point, Floneum plugins have been limited to reading the content websites initially provide without interacting with the page. Some websites require you to interact with the page to get interesting content. In the 0.2 release, we have added the ability to control a browser (or hidden browser) in Floneum. This makes it possible for workflows built with Floneum to access much more content from web pages and even create new content:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::video::TAG_NAME,
                        namespace: dioxus_elements::video::NAME_SPACE,
                        attrs: &[
                            dioxus_core::TemplateAttribute::Static {
                                name: dioxus_elements::video::padding.0,
                                namespace: dioxus_elements::video::padding.1,
                                value: "5px",
                            },
                            dioxus_core::TemplateAttribute::Dynamic { id: 2usize },
                            dioxus_core::TemplateAttribute::Dynamic { id: 3usize },
                            dioxus_core::TemplateAttribute::Dynamic { id: 4usize },
                        ],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::source::TAG_NAME,
                                namespace: dioxus_elements::source::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::source::src.0,
                                        namespace: dioxus_elements::source::src.1,
                                        value: "/assets/headless_browser.mp4",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::source::r#type.0,
                                        namespace: dioxus_elements::source::r#type.1,
                                        value: "video/mp4",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "sharing-workflows-on-the-cloud",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#sharing-workflows-on-the-cloud",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Sharing Workflows on the Cloud",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Originally, Floneum workflows needed to be shared as a file and copied from and to a specific directory. In the latest release of Floneum, you can just save a workflow to the cloud, get an unique workflow number, and load that workflow on any computer easily!"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::class.0,
                            namespace: dioxus_elements::div::class.1,
                            value: "flex flex-row",
                        }],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::video::TAG_NAME,
                                    namespace: dioxus_elements::video::NAME_SPACE,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: dioxus_elements::video::padding.0,
                                            namespace: dioxus_elements::video::padding.1,
                                            value: "5px",
                                        },
                                        dioxus_core::TemplateAttribute::Static {
                                            name: dioxus_elements::video::width.0,
                                            namespace: dioxus_elements::video::width.1,
                                            value: "50%",
                                        },
                                        dioxus_core::TemplateAttribute::Dynamic { id: 5usize },
                                        dioxus_core::TemplateAttribute::Dynamic { id: 6usize },
                                        dioxus_core::TemplateAttribute::Dynamic { id: 7usize },
                                    ],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::source::TAG_NAME,
                                            namespace: dioxus_elements::source::NAME_SPACE,
                                            attrs: &[
                                                dioxus_core::TemplateAttribute::Static {
                                                    name: dioxus_elements::source::src.0,
                                                    namespace: dioxus_elements::source::src.1,
                                                    value: "/assets/save_cloud.mp4",
                                                },
                                                dioxus_core::TemplateAttribute::Static {
                                                    name: dioxus_elements::source::r#type.0,
                                                    namespace: dioxus_elements::source::r#type.1,
                                                    value: "video/mp4",
                                                },
                                            ],
                                            children: &[],
                                        }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::video::TAG_NAME,
                                    namespace: dioxus_elements::video::NAME_SPACE,
                                    attrs: &[
                                        dioxus_core::TemplateAttribute::Static {
                                            name: dioxus_elements::video::padding.0,
                                            namespace: dioxus_elements::video::padding.1,
                                            value: "5px",
                                        },
                                        dioxus_core::TemplateAttribute::Static {
                                            name: dioxus_elements::video::width.0,
                                            namespace: dioxus_elements::video::width.1,
                                            value: "50%",
                                        },
                                        dioxus_core::TemplateAttribute::Dynamic { id: 8usize },
                                        dioxus_core::TemplateAttribute::Dynamic { id: 9usize },
                                        dioxus_core::TemplateAttribute::Dynamic { id: 10usize },
                                    ],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::source::TAG_NAME,
                                            namespace: dioxus_elements::source::NAME_SPACE,
                                            attrs: &[
                                                dioxus_core::TemplateAttribute::Static {
                                                    name: dioxus_elements::source::src.0,
                                                    namespace: dioxus_elements::source::src.1,
                                                    value: "/assets/load_cloud.mp4",
                                                },
                                                dioxus_core::TemplateAttribute::Static {
                                                    name: dioxus_elements::source::r#type.0,
                                                    namespace: dioxus_elements::source::r#type.1,
                                                    value: "video/mp4",
                                                },
                                            ],
                                            children: &[],
                                        }
                                    }],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "plugin-examples",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#plugin-examples",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Plugin Examples",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Floneum aims to make development of workflows simple. As part of that, clear and concise documentation is very important to us. As part of the new release, each plugin now has an example of the plugin's usage along side the documentation in the current node view. You can click on the example to get a quick overview of inputs and outputs for a specific node."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 11usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Plugin Example",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "rewritten-ui",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#rewritten-ui",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Rewritten UI!",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "As you may have noticed, the UI has been entirely rewritten for the 0.2 release. Previously Floneum was written in "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://lib.rs/crates/egui",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "EGUI" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ", an immediate mode GUI library for rust. We initially chose EGUI to take advantage of "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/setzer22/egui_node_graph",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "a node graph library written for EGUI" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ", but we ran into issues with layouts in EGUI and limitations with the node graph library. Because of this, Floneum has switched to "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/DioxusLabs/dioxus",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Dioxus" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " with a custom node graph library. Along with this change comes a range of improvements to the UI enabled by the flexibly our own implementation offers. For example previously list types could only take individual elements now they can combine multiple lists and elements together. Another major improvement is moving the inputs and outputs of a node into a sidebar on the right. This makes it easier to see and edit large blocks of text that a node generates."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "package-manger",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#package-manger",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Package Manger",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "When Floneum was originally released, it looked for plugins within a specific github repo which distributing community made plugins difficult. The new release makes publishing a plugin as easy as creating a github repo and adding a tag. Floneum will automatically find any repos with the tag  "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "floneum" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " and a build generated by the CLI and add them to the plugin registry."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "introducing-the-floneum-cli",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#introducing-the-floneum-cli",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Introducing the Floneum CLI",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "As part of the package manager, Floneum has a new CLI to make it easier to build and distribute plugins! You can install the CLI by "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://www.rust-lang.org/tools/install",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "installing rust" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                { text : " and then running:" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">cargo install --git https:</span><span style=\"color:#65737e;\">//github.com/floneum/floneum floneum-cli\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Or downloading the CLI from the release page.",
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Once you have the CLI installed, you can build your Floneum plugins by running:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">floneum build\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "That's it! Just tag your github repo with  ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::code::TAG_NAME,
                                    namespace: dioxus_elements::code::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "floneum",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " and your plugin will be available in Floneum!",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: p :: TAG_NAME,
                        namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Bonus: You can also clear the package cache with the CLI by running  "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "floneum clear" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " to get the latest plugins before the cache expires."
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "keep-up-to-date-with-nightly-builds",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#keep-up-to-date-with-nightly-builds",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Keep Up to Date with Nightly Builds",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "If you want to keep up with the latest developments of Floneum, nightly builds are now created for "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/actions/runs/6102318885",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "every commit" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " to the main branch. You can download the latest nightly build from the CI of the main branch:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 12usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Nightly Build Files",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: p :: TAG_NAME,
                        namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Updates and discussions about development also happen on the "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://discord.gg/dQdmhuB8q5",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "Floneum Discord" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : "." }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "what-can-you-build-with-floneum",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#what-can-you-build-with-floneum",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "What can you build with Floneum?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "While Floneum is still in early development, it is already possible to build a range of workflows with it. Here are some examples of workflows that can be built with the latest release of Floneum:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "news-feed-summarization",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#news-feed-summarization",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "News feed summarization!",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 13usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "News Summarization Example",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: h3 :: TAG_NAME, namespace
                : dioxus_elements :: h3 :: NAME_SPACE, attrs : &
                [dioxus_core :: TemplateAttribute :: Static
                {
                    name : dioxus_elements :: h3 :: id.0, namespace :
                    dioxus_elements :: h3 :: id.1, value :
                    "combining-multiple-models-to-create-an-efficient-question-answering-workflow-sources-relevant-to-your-use-case",
                }], children : &
                [{
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "#combining-multiple-models-to-create-an-efficient-question-answering-workflow-sources-relevant-to-your-use-case",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: class.0, namespace :
                            dioxus_elements :: a :: class.1, value : "header",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Combining multiple models to create an efficient question answering workflow sources relevant to your use case!"
                        }],
                    }
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 14usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Question Answering Example",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "automatic-content-creation-with-browser-automation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value:
                                            "#automatic-content-creation-with-browser-automation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Automatic content creation with browser automation!",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::video::TAG_NAME,
                        namespace: dioxus_elements::video::NAME_SPACE,
                        attrs: &[
                            dioxus_core::TemplateAttribute::Static {
                                name: dioxus_elements::video::padding.0,
                                namespace: dioxus_elements::video::padding.1,
                                value: "5px",
                            },
                            dioxus_core::TemplateAttribute::Dynamic { id: 15usize },
                            dioxus_core::TemplateAttribute::Dynamic { id: 16usize },
                            dioxus_core::TemplateAttribute::Dynamic { id: 17usize },
                        ],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::source::TAG_NAME,
                                namespace: dioxus_elements::source::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::source::src.0,
                                        namespace: dioxus_elements::source::src.1,
                                        value: "/assets/headless_browsing_example.mp4",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::source::r#type.0,
                                        namespace: dioxus_elements::source::r#type.1,
                                        value: "video/mp4",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "how-do-i-get-started",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#how-do-i-get-started",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "How do I get started?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "To get started, just go to " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/releases/tag/v0.2.0",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "the downloads page" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ", download the installer and create your first workflow (or try an example). That is it! No need to install python, or cuda!"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "If you want to learn more about Floneum, check out the ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://floneum.com/docs/user",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "user documentation",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: " or " },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://floneum.com/docs/developer",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "plugin developer documentation",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: "." },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "If you have any questions, feel free to join the ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://discord.gg/dQdmhuB8q5",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Floneum Discord",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " and ask away!",
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Join the discussion:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://www.reddit.com/r/LocalLLaMA/comments/16gusz2/floneum_02_released_open_source_local_graph/",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "Reddit" }],
                            }
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[{
                                        dioxus_core::TemplateNode::Element {
                                            tag: dioxus_elements::elements::a::TAG_NAME,
                                            namespace: dioxus_elements::a::NAME_SPACE,
                                            attrs: &[dioxus_core::TemplateAttribute::Static {
                                                name: dioxus_elements::a::href.0,
                                                namespace: dioxus_elements::a::href.1,
                                                value:
                                                    "https://news.ycombinator.com/item?id=37481027",
                                            }],
                                            children: &[dioxus_core::TemplateNode::Text {
                                                text: "HN",
                                            }],
                                        }
                                    }],
                                }
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[
                &[6u8, 0u8],
                &[9u8, 0u8],
                &[12u8],
                &[12u8],
                &[12u8],
                &[15u8, 0u8],
                &[15u8, 0u8],
                &[15u8, 0u8],
                &[15u8, 1u8],
                &[15u8, 1u8],
                &[15u8, 1u8],
                &[18u8, 0u8],
                &[33u8, 0u8],
                &[38u8, 0u8],
                &[40u8, 0u8],
                &[42u8],
                &[42u8],
                &[42u8],
            ],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(
                None,
                ___TEMPLATE,
                Box::new([]),
                Box::new([
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/get_article_demo.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/rss_demo.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::controls.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "21")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::controls.1,
                            dioxus_elements::video::controls.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::autoplay.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "22")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::autoplay.1,
                            dioxus_elements::video::autoplay.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::muted.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "23")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::muted.1,
                            dioxus_elements::video::muted.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::controls.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "32")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::controls.1,
                            dioxus_elements::video::controls.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::autoplay.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "33")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::autoplay.1,
                            dioxus_elements::video::autoplay.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::muted.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "34")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::muted.1,
                            dioxus_elements::video::muted.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::controls.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "39")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::controls.1,
                            dioxus_elements::video::controls.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::autoplay.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "40")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::autoplay.1,
                            dioxus_elements::video::autoplay.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::muted.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "41")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::muted.1,
                            dioxus_elements::video::muted.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/plugin_examples.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/nightly_builds.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/news_summary.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/question_answer_example.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::controls.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "94")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::controls.1,
                            dioxus_elements::video::controls.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::autoplay.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "95")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::autoplay.1,
                            dioxus_elements::video::autoplay.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::video::muted.0,
                            {
                                #[cfg(debug_assertions)]
                                {
                                    _ = true;
                                    GlobalSignal::with_key(|| true, {
                                        concat!(file!(), ":", line!(), ":", column!(), ":", "96")
                                    })
                                    .maybe_with_rt(|s| s.clone())
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    true.clone()
                                }
                            },
                            dioxus_elements::video::muted.1,
                            dioxus_elements::video::muted.2,
                        )
                    }]),
                ]),
            );
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 96usize),
            roots: &[
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: h1 :: TAG_NAME, namespace
                : dioxus_elements :: h1 :: NAME_SPACE, attrs : &
                [dioxus_core :: TemplateAttribute :: Static
                {
                    name : dioxus_elements :: h1 :: id.0, namespace :
                    dioxus_elements :: h1 :: id.1, value :
                    "kalosm-v020-tasks-evaluation-prompt-auto-tuning-regex-validation-surreal-database-integration-rag-improvements-performance-improvements-and-more",
                }], children : &
                [{
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "#kalosm-v020-tasks-evaluation-prompt-auto-tuning-regex-validation-surreal-database-integration-rag-improvements-performance-improvements-and-more",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: class.0, namespace :
                            dioxus_elements :: a :: class.1, value : "header",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Kalosm v0.2.0 Tasks, Evaluation, Prompt Auto-Tuning, Regex Validation, Surreal Database Integration, RAG improvements, Performance Improvements, and More!"
                        }],
                    }
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "We're excited to announce the release of Kalosm v0.2.0! This release includes a number of new features, improvements, and bug fixes including:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Tasks and Agents",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Task Evaluation",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Prompt Auto-Tuning",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Regex Validation",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Surreal Database Integration",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "RAG improvements",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Performance Improvements",
                                    }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "New Models",
                                    }],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "tasks-and-agents",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#tasks-and-agents",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Tasks and Agents!",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm now includes utilities for running, evaluating, and improving tasks and agents."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Let's build a simple task and agent to demonstrate the new functionality."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">\t</span><span style=\"color:#65737e;\">// You can use tasks with llama, mistral or phi models.\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> llm = Phi::new_chat();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Now we can create a task. We will create a simple assistant that identifies keywords in a sentence.\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> task = Task::builder(&amp;llm, &quot;</span><span style=\"color:#a3be8c;\">You are an assistant who identifies keywords in a sentence. When identifying keywords, you will output the keywords in a comma-separated list.</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">\t\t</span><span style=\"color:#65737e;\">// You can optionally add constraints to the task.\n</span><span style=\"color:#c0c5ce;\">        </span><span style=\"color:#65737e;\">// accept 2-5 keywords of 2-15 characters each\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">with_constraints</span><span style=\"color:#c0c5ce;\">(RegexParser::new(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#c0c5ce;\">#</span><span style=\"color:#a3be8c;\">&quot;The keywords are:([ a-z]{2,15},){2,5}</span><span style=\"color:#c0c5ce;\">&quot;#).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">())\n</span><span style=\"color:#c0c5ce;\">\t\t</span><span style=\"color:#65737e;\">/// You can also add examples to the task to help the agent learn how to solve math problems.\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">with_example</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is the weather like in New York?</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">The keywords are: weather, new york, </span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">with_example</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is the capital of France?</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">The keywords are: capital, france, </span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// The first time we use the task, it will load the model and prompt which will take a bit longer.\n</span><span style=\"color:#c0c5ce;\">    task.</span><span style=\"color:#96b5b4;\">run</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is the temperature in Chicago?</span><span style=\"color:#c0c5ce;\">&quot;, &amp;llm)\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Or you can use more complex constraints to ",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#65737e;\">// First, we need to create a Llama instance. We will use the default chat model (open chat) for this example. Tasks work with chat and text generation models.\n</span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> llm = Llama::new_chat();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Next (optionally), we can define constraints for the task. In this example, we will use a regex to validate the output of the task.\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> constraints =\n</span><span style=\"color:#c0c5ce;\">\tRegexParser::new(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#a3be8c;\">&quot;(Step \\d: \\d+ [+\\-*/] \\d+ = \\d+\\n){1,3}Output: \\d+</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Now we can create a task. We will create a simple math problem solving task.\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> task = Task::builder(&amp;llm, &quot;</span><span style=\"color:#a3be8c;\">You are an assistant who solves math problems. When solving problems, you will always solve problems step by step with one step per line. Once you have solved the problem, you will output the result in the format &#39;Output: &lt;result&gt;&#39;.</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">with_constraints</span><span style=\"color:#c0c5ce;\">(constraints)\n</span><span style=\"color:#c0c5ce;\">\t</span><span style=\"color:#65737e;\">// You can also add examples to the task to help the agent learn how to solve math problems.\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">with_example</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is 1 + 2?</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Step 1: 1 + 2 = 3</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">Output: 3</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">with_example</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is 3 + 4?</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Step 1: 3 + 4 = 7</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">Output: 7</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">with_example</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is (4 + 8) / 3?</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Step 1: 4 + 8 = 12</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">Step 2: 12 / 3 = 4</span><span style=\"color:#96b5b4;\">\\n</span><span style=\"color:#a3be8c;\">Output: 4</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// The first time we use the task, it will load the model and prompt which will take a bit longer.\n</span><span style=\"color:#c0c5ce;\">task.</span><span style=\"color:#96b5b4;\">run</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">What is 2 + 2?</span><span style=\"color:#c0c5ce;\">&quot;, &amp;llm)\n</span><span style=\"color:#c0c5ce;\">\t.await\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">\t.await\n</span><span style=\"color:#c0c5ce;\">\t.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Tasks can efficiently reuse the session between runs, which can significantly speed up the process:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">question </span><span style=\"color:#d08770;\">1\n</span><span style=\"color:#c0c5ce;\">Step </span><span style=\"color:#d08770;\">1</span><span style=\"color:#c0c5ce;\">: </span><span style=\"color:#d08770;\">2 </span><span style=\"color:#c0c5ce;\">+ </span><span style=\"color:#d08770;\">2 </span><span style=\"color:#c0c5ce;\">= </span><span style=\"color:#d08770;\">4\n</span><span style=\"color:#c0c5ce;\">Output: </span><span style=\"color:#d08770;\">4.\n</span><span style=\"color:#c0c5ce;\">first question took: </span><span style=\"color:#d08770;\">18.371939709</span><span style=\"color:#c0c5ce;\">s\n</span><span style=\"color:#c0c5ce;\">question </span><span style=\"color:#d08770;\">2\n</span><span style=\"color:#c0c5ce;\">Step </span><span style=\"color:#d08770;\">1</span><span style=\"color:#c0c5ce;\">: </span><span style=\"color:#d08770;\">4 </span><span style=\"color:#c0c5ce;\">+ </span><span style=\"color:#d08770;\">4 </span><span style=\"color:#c0c5ce;\">= </span><span style=\"color:#d08770;\">8\n</span><span style=\"color:#c0c5ce;\">Output: </span><span style=\"color:#d08770;\">8\n</span><span style=\"color:#c0c5ce;\">second question took: </span><span style=\"color:#d08770;\">5.723529959</span><span style=\"color:#c0c5ce;\">s\n</span><span style=\"color:#c0c5ce;\">question </span><span style=\"color:#d08770;\">3\n</span><span style=\"color:#c0c5ce;\">Step </span><span style=\"color:#d08770;\">1</span><span style=\"color:#c0c5ce;\">: </span><span style=\"color:#d08770;\">7 </span><span style=\"color:#c0c5ce;\">+ </span><span style=\"color:#d08770;\">5 </span><span style=\"color:#c0c5ce;\">= </span><span style=\"color:#d08770;\">12\n</span><span style=\"color:#c0c5ce;\">Step </span><span style=\"color:#d08770;\">2</span><span style=\"color:#c0c5ce;\">: </span><span style=\"color:#d08770;\">12 </span><span style=\"color:#c0c5ce;\">/ </span><span style=\"color:#d08770;\">2 </span><span style=\"color:#c0c5ce;\">= </span><span style=\"color:#d08770;\">6\n</span><span style=\"color:#c0c5ce;\">Output: </span><span style=\"color:#d08770;\">6\n</span><span style=\"color:#c0c5ce;\">third question took: </span><span style=\"color:#d08770;\">9.303650625</span><span style=\"color:#c0c5ce;\">s\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "The third question required a more complex calculation and took more tokens to solve, but the time to solve the question was still significantly faster than the first question. The session from the first question was reused for the second and third questions which made the second and third questions run faster."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: strong :: TAG_NAME,
                                namespace : dioxus_elements :: strong :: NAME_SPACE, attrs :
                                & [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "Evaluation Abstraction:" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " Introducing an evaluation abstraction, providing enhanced functionality. ("
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/pull/113",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "#113" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : ")" }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">+-----------------+-------+\n</span><span style=\"color:#c0c5ce;\">| Statistic       | Value |\n</span><span style=\"color:#c0c5ce;\">+=========================+\n</span><span style=\"color:#c0c5ce;\">| Mean            | </span><span style=\"color:#d08770;\">0.75  </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|-----------------+-------|\n</span><span style=\"color:#c0c5ce;\">| Median          | </span><span style=\"color:#d08770;\">0.77  </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|-----------------+-------|\n</span><span style=\"color:#c0c5ce;\">| Min             | </span><span style=\"color:#d08770;\">0.49  </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|-----------------+-------|\n</span><span style=\"color:#c0c5ce;\">| Max             | </span><span style=\"color:#d08770;\">0.94  </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|-----------------+-------|\n</span><span style=\"color:#c0c5ce;\">| 25th Percentile | </span><span style=\"color:#d08770;\">0.67  </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|-----------------+-------|\n</span><span style=\"color:#c0c5ce;\">| 75th Percentile | </span><span style=\"color:#d08770;\">0.81  </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">+-----------------+-------+\n</span><span style=\"color:#c0c5ce;\">+------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------+\n</span><span style=\"color:#c0c5ce;\">| Expected Output                                                                          | Actual Output                                                                             | Score              |\n</span><span style=\"color:#c0c5ce;\">+===========================================================================================================================================================================================================+\n</span><span style=\"color:#c0c5ce;\">| What are Floneum plugins?                                                                | What is designed to support an expanding ecosystem of plugins?                            | </span><span style=\"color:#d08770;\">0.49 </span><span style=\"color:#c0c5ce;\">(low outlier) |\n</span><span style=\"color:#c0c5ce;\">|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|\n</span><span style=\"color:#c0c5ce;\">| What is supervised learning in machine learning?                                         | What is required </span><span style=\"color:#b48ead;\">for</span><span style=\"color:#c0c5ce;\"> machine learning models in order to make accurate predictions?       | </span><span style=\"color:#d08770;\">0.50 </span><span style=\"color:#c0c5ce;\">(low outlier) |\n</span><span style=\"color:#c0c5ce;\">|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|\n</span><span style=\"color:#c0c5ce;\">| What distinguishes open-source software from proprietary software?                       | What </span><span style=\"color:#b48ead;\">type </span><span style=\"color:#c0c5ce;\">of access does open source software provide </span><span style=\"color:#b48ead;\">for</span><span style=\"color:#c0c5ce;\"> its users?                      | </span><span style=\"color:#d08770;\">0.64               </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|\n</span><span style=\"color:#c0c5ce;\">| How does blockchain contribute to the security of cryptocurrencies?                      | What is the advantage of cryptocurrency over traditional currencies in terms of security? | </span><span style=\"color:#d08770;\">0.65               </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|\n</span><span style=\"color:#c0c5ce;\">| What are the tradeoffs of using chat </span><span style=\"color:#d08770;\">GPT</span><span style=\"color:#c0c5ce;\">?                                                | What is an example of how using ChatGPT may </span><span style=\"background-color:#bf616a;color:#2b303b;\">become</span><span style=\"color:#c0c5ce;\"> challenging in certain situations?     | </span><span style=\"color:#d08770;\">0.65               </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|\n</span><span style=\"color:#c0c5ce;\">| What is the relationship between DevOps, continuous integration and continuous delivery? | What is the main goal of DevOps?                                                          | </span><span style=\"color:#d08770;\">0.67               </span><span style=\"color:#c0c5ce;\">|\n</span><span style=\"color:#c0c5ce;\">|------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------|\n</span><span style=\"color:#c0c5ce;\">| ... </span><span style=\"color:#d08770;\">18</span><span style=\"color:#c0c5ce;\"> more                                                                              |                                                                                           | </span><span style=\"color:#d08770;\">0.80 </span><span style=\"color:#c0c5ce;\">(average)     |\n</span><span style=\"color:#c0c5ce;\">+------------------------------------------------------------------------------------------+-------------------------------------------------------------------------------------------+--------------------+\n</span><span style=\"color:#c0c5ce;\">| Score Histogram        |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.00 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.10</span><span style=\"color:#c0c5ce;\">:           |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.10 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.20</span><span style=\"color:#c0c5ce;\">:           |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.20 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.30</span><span style=\"color:#c0c5ce;\">:           |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.30 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.40</span><span style=\"color:#c0c5ce;\">:           |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.40 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.50</span><span style=\"color:#c0c5ce;\">: *         |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.50 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.60</span><span style=\"color:#c0c5ce;\">: *         |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.60 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.70</span><span style=\"color:#c0c5ce;\">: ******    |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.70 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.80</span><span style=\"color:#c0c5ce;\">: ********* |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.80 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">0.90</span><span style=\"color:#c0c5ce;\">: *****     |\n</span><span style=\"color:#c0c5ce;\">| </span><span style=\"color:#d08770;\">0.90 </span><span style=\"color:#c0c5ce;\">- </span><span style=\"color:#d08770;\">1.00</span><span style=\"color:#c0c5ce;\">: **        |\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: strong :: TAG_NAME,
                                namespace : dioxus_elements :: strong :: NAME_SPACE, attrs :
                                & [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "Prompt Auto-Tuning:" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " Prompts can now be automatically tuned for better performance. ("
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://github.com/floneum/floneum/pull/132",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "#132" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : ")" }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Lets take a look at the new prompt auto-tuning feature with an example. As part of the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "#Improved-Chunking-Strategies",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "RAG improvements" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ", kalosm includes a task that generates hypothetical questions about a text for an embedding model that can be used to find similar documents based on the meaning of the text for the  section. We can tune that task to find the best examples for the task with a "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "PromptAnnealer" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ":" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">const </span><span style=\"color:#d08770;\">EXAMPLES</span><span style=\"color:#c0c5ce;\">: &amp;[(&amp;</span><span style=\"color:#b48ead;\">str</span><span style=\"color:#c0c5ce;\">, &amp;</span><span style=\"color:#b48ead;\">str</span><span style=\"color:#c0c5ce;\">)]= &amp;[\n</span><span style=\"color:#c0c5ce;\">    (&quot;</span><span style=\"color:#a3be8c;\">An example input to the task</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">An example output to the task</span><span style=\"color:#c0c5ce;\">&quot;),\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// ...more examples\n</span><span style=\"color:#c0c5ce;\">];\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> llm = Phi::v2();\n</span><span style=\"color:#b48ead;\">const </span><span style=\"color:#d08770;\">PREFIX</span><span style=\"color:#c0c5ce;\">: &amp;</span><span style=\"color:#b48ead;\">str </span><span style=\"color:#c0c5ce;\">= &quot;</span><span style=\"color:#a3be8c;\">Questions that are answered by the previous text: </span><span style=\"color:#c0c5ce;\">&quot;;\n</span><span style=\"color:#b48ead;\">const </span><span style=\"color:#d08770;\">QUESTION_STARTERS</span><span style=\"color:#c0c5ce;\">: [&amp;</span><span style=\"color:#b48ead;\">str</span><span style=\"color:#c0c5ce;\">; </span><span style=\"color:#d08770;\">9</span><span style=\"color:#c0c5ce;\">] = [\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">Who</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">What</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">When</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Where</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Why</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">How</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Which</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Whom</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Whose</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">];\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> constraints = LiteralParser::new(</span><span style=\"color:#d08770;\">PREFIX</span><span style=\"color:#c0c5ce;\">).</span><span style=\"color:#96b5b4;\">then</span><span style=\"color:#c0c5ce;\">(\n</span><span style=\"color:#c0c5ce;\">    IndexParser::new(\n</span><span style=\"color:#c0c5ce;\">        </span><span style=\"color:#d08770;\">QUESTION_STARTERS\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">iter</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">copied</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">map</span><span style=\"color:#c0c5ce;\">(LiteralParser::new)\n</span><span style=\"color:#c0c5ce;\">            .collect::&lt;Vec&lt;_&gt;&gt;(),\n</span><span style=\"color:#c0c5ce;\">    )\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">then</span><span style=\"color:#c0c5ce;\">(StopOn::new(&quot;</span><span style=\"color:#a3be8c;\">?</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">filter_characters</span><span style=\"color:#c0c5ce;\">(\n</span><span style=\"color:#c0c5ce;\">        |</span><span style=\"color:#bf616a;\">c</span><span style=\"color:#c0c5ce;\">| matches!(c, &#39; &#39; | &#39;</span><span style=\"color:#a3be8c;\">?</span><span style=\"color:#c0c5ce;\">&#39; | &#39;</span><span style=\"color:#a3be8c;\">a</span><span style=\"color:#c0c5ce;\">&#39;..=&#39;</span><span style=\"color:#a3be8c;\">z</span><span style=\"color:#c0c5ce;\">&#39; | &#39;</span><span style=\"color:#a3be8c;\">A</span><span style=\"color:#c0c5ce;\">&#39;..=&#39;</span><span style=\"color:#a3be8c;\">Z</span><span style=\"color:#c0c5ce;\">&#39; | &#39;</span><span style=\"color:#a3be8c;\">0</span><span style=\"color:#c0c5ce;\">&#39;..=&#39;</span><span style=\"color:#a3be8c;\">9</span><span style=\"color:#c0c5ce;\">&#39; | &#39;</span><span style=\"color:#a3be8c;\">,</span><span style=\"color:#c0c5ce;\">&#39;),\n</span><span style=\"color:#c0c5ce;\">    ))\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">repeat</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">1</span><span style=\"color:#c0c5ce;\">..=</span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">),\n</span><span style=\"color:#c0c5ce;\">);\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> task = Task::builder(&quot;</span><span style=\"color:#a3be8c;\">You generate hypothetical questions that may be answered by the given text. The questions restate any information necessary to understand the question</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_constraints</span><span style=\"color:#c0c5ce;\">(constraints);\n</span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> annealing = kalosm::PromptAnnealer::builder(&amp;llm, </span><span style=\"color:#d08770;\">EXAMPLES</span><span style=\"color:#c0c5ce;\">, task)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_initial_temperature</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">0.6</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">with_initial_choice_range</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#d08770;\">1</span><span style=\"color:#c0c5ce;\">..</span><span style=\"color:#d08770;\">4</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .await;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> result = annealing.</span><span style=\"color:#96b5b4;\">run</span><span style=\"color:#c0c5ce;\">().await;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">println!(&quot;</span><span style=\"color:#a3be8c;\">Result: </span><span style=\"color:#d08770;\">{:?}</span><span style=\"color:#c0c5ce;\">&quot;, result);\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Here is the best set of examples that the prompt annealer found for the task:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::table::TAG_NAME,
                        namespace: dioxus_elements::table::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::thead::TAG_NAME,
                                    namespace: dioxus_elements::thead::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::th::TAG_NAME,
                                                namespace: dioxus_elements::th::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "Input",
                                                }],
                                            }
                                        },
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::th::TAG_NAME,
                                                namespace: dioxus_elements::th::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "Output",
                                                }],
                                            }
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::tr::TAG_NAME,
                                    namespace: dioxus_elements::tr::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: th :: TAG_NAME,
                                namespace : dioxus_elements :: th :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "While traditional databases rely on a fixed schema, NoSQL databases like MongoDB offer a flexible structure, allowing you to store and retrieve data in a more dynamic way. This flexibility is particularly beneficial for applications with evolving data requirements."
                                }],
                            }
                                        },
                                        {
                                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: th :: TAG_NAME,
                                namespace : dioxus_elements :: th :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "\"How does MongoDB differ from traditional databases?"
                                }],
                            }
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::tr::TAG_NAME,
                                    namespace: dioxus_elements::tr::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: th :: TAG_NAME,
                                namespace : dioxus_elements :: th :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "Blockchain technology, beyond cryptocurrencies, is being explored for applications like smart contracts. Smart contracts are self-executing contracts with the terms of the agreement directly written into code."
                                }],
                            }
                                        },
                                        {
                                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: th :: TAG_NAME,
                                namespace : dioxus_elements :: th :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                {
                                    text :
                                    "\"How is blockchain technology utilized in the concept of smart contracts?"
                                }],
                            }
                                        },
                                    ],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Feeding those two examples into the task achieves a similarity score of 0.71 for all of the other examples compared to choosing two random examples from the task which only achieves a similarity score of  0.62 for all of the other examples."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "regex-validation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#regex-validation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Regex Validation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Some feedback we got from the initial release of kalosm, was that constraints for constrained generation was too complex. Constraints in Kalosm serve two purposes:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ol::TAG_NAME,
                        namespace: dioxus_elements::ol::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Validation: Constraints can be used to validate the output of the model. The model will only output text that can be parsed by the constraints. This lets you ensure that the output of the model is in the format you expect."
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "For example, you may want to force the model response to always start with a prefix that guides the model:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> llm = Llama::default();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> prompt = &quot;</span><span style=\"color:#a3be8c;\">Generate a list of 10 words you use to describe yourself: </span><span style=\"color:#c0c5ce;\">&quot;;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> validator = LiteralParser::new(&quot;</span><span style=\"color:#a3be8c;\">(Responding as a pirate) </span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">then</span><span style=\"color:#c0c5ce;\">(OneLine);\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> stream = llm.</span><span style=\"color:#96b5b4;\">stream_structured_text</span><span style=\"color:#c0c5ce;\">(prompt, validator).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    stream.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ol::TAG_NAME,
                        namespace: dioxus_elements::ol::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Parsing: Constraints can be used to parse the output of the model. This can be extremely useful when you want to generate a specific structure from an LLM without writing separate logic for validation and parsing."
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "For example, you may want to generate a list of 10 numbers:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> llm = Llama::default();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> prompt = &quot;</span><span style=\"color:#a3be8c;\">Prime numbers: </span><span style=\"color:#c0c5ce;\">&quot;;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> validator = &lt;[</span><span style=\"color:#b48ead;\">f32</span><span style=\"color:#c0c5ce;\">; </span><span style=\"color:#d08770;\">10</span><span style=\"color:#c0c5ce;\">] as HasParser&gt;::new_parser();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> words = llm.</span><span style=\"color:#96b5b4;\">stream_structured_text</span><span style=\"color:#c0c5ce;\">(prompt, validator).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> result: [</span><span style=\"color:#b48ead;\">f32</span><span style=\"color:#c0c5ce;\">; </span><span style=\"color:#d08770;\">10</span><span style=\"color:#c0c5ce;\">] = words.</span><span style=\"color:#96b5b4;\">result</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    println!(&quot;</span><span style=\"color:#a3be8c;\">Prime numbers: </span><span style=\"color:#d08770;\">{:?}</span><span style=\"color:#c0c5ce;\">&quot;, result);\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "If you only need to validate the output of the model, the existing constraints can be more complex than what you need. In this release, we've added support for regex validation. This makes it easier to validate the output of the model without handling parsing:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> llm = Llama::default();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> prompt = &quot;</span><span style=\"color:#a3be8c;\">Generate a list of 10 words you use to describe yourself: </span><span style=\"color:#c0c5ce;\">&quot;;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> validator = RegexParser::new(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#a3be8c;\">&quot;\\(Responding as a pirate\\) ([a-z]{1,10}, ){10}</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> stream = llm.</span><span style=\"color:#96b5b4;\">stream_structured_text</span><span style=\"color:#c0c5ce;\">(prompt, validator).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    stream.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "surreal-database-integration",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#surreal-database-integration",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Surreal Database Integration",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Vector databases can be very useful when combined with LLMs. They can be used to store and retrieve similar documents based on the meaning of the text, not just the words used. However, vector databases only handle a very limited number of use cases. In this release, we've added support for Surreal DB for more traditional database use cases. Surreal DB can be embedded into your application and used to store and retrieve data locally as well as over the network."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Kalosm 0.2 allows you to create tables within Surreal DB that are indexed by vectors. You can then insert documents (or other embeddings) into the table and query the table for similar documents based on the meaning of the text."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Let's take a look at how you can use the Surreal DB integration to store and retrieve similar documents based on the meaning of the text:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">comfy_table::{Cell, Color, Row, Table};\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::*;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">std::path::PathBuf;\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">surrealdb::{engine::local::RocksDb, Surreal};\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> exists = std::path::Path::new(&quot;</span><span style=\"color:#a3be8c;\">./db</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">exists</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create or open a new database at ./db/temp.db\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> db = Surreal::new::&lt;RocksDb&gt;(&quot;</span><span style=\"color:#a3be8c;\">./db/temp.db</span><span style=\"color:#c0c5ce;\">&quot;).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Select a specific namespace / database within the database\n</span><span style=\"color:#c0c5ce;\">db.</span><span style=\"color:#96b5b4;\">use_ns</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">test</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">use_db</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">test</span><span style=\"color:#c0c5ce;\">&quot;).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Create a new document table that uses arroy for fast vector search\n</span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> document_table = db\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">document_table_builder</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">documents</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Store the embedding database in the same directory as the document table\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">at</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">./db/embeddings.db</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">    .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// If the database doesn&#39;t exist, create it and insert some documents\n</span><span style=\"color:#b48ead;\">if </span><span style=\"color:#c0c5ce;\">!exists {\n</span><span style=\"color:#c0c5ce;\">    std::fs::create_dir_all(&quot;</span><span style=\"color:#a3be8c;\">documents</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Load some files from a directory\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> documents = DocumentFolder::try_from(PathBuf::from(&quot;</span><span style=\"color:#a3be8c;\">./documents</span><span style=\"color:#c0c5ce;\">&quot;)).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Convert the folder into a vector of documents\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> documents = documents.</span><span style=\"color:#96b5b4;\">into_documents</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">for</span><span style=\"color:#c0c5ce;\"> document in documents {\n</span><span style=\"color:#c0c5ce;\">        </span><span style=\"color:#65737e;\">// And insert the documents into the table (this will automatically chunk and embed the documents before they are inserted into the table and vector database)\n</span><span style=\"color:#c0c5ce;\">        document_table.</span><span style=\"color:#96b5b4;\">insert</span><span style=\"color:#c0c5ce;\">(document).await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    }\n</span><span style=\"color:#c0c5ce;\">}\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#65737e;\">// Now we can query the table for similar documents based on the meaning of the text\n</span><span style=\"color:#b48ead;\">loop </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Get a query from the user and embed that query into a vector\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> user_question = </span><span style=\"color:#96b5b4;\">prompt_input</span><span style=\"color:#c0c5ce;\">(&quot;</span><span style=\"color:#a3be8c;\">Query: </span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> user_question_embedding = document_table\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">embedding_model_mut</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">embed</span><span style=\"color:#c0c5ce;\">(&amp;user_question)\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Select the 5 most similar documents to the user&#39;s query\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> nearest_5 = document_table\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">select_nearest_embedding</span><span style=\"color:#c0c5ce;\">(user_question_embedding, </span><span style=\"color:#d08770;\">5</span><span style=\"color:#c0c5ce;\">)\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Display the results in a formatted table with colors based on the distance from the query\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> table = Table::new();\n</span><span style=\"color:#c0c5ce;\">    table.</span><span style=\"color:#96b5b4;\">set_header</span><span style=\"color:#c0c5ce;\">(vec![&quot;</span><span style=\"color:#a3be8c;\">Score</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">Value</span><span style=\"color:#c0c5ce;\">&quot;]);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">for</span><span style=\"color:#c0c5ce;\"> result in nearest_5 {\n</span><span style=\"color:#c0c5ce;\">        </span><span style=\"color:#b48ead;\">let mut</span><span style=\"color:#c0c5ce;\"> row = Row::new();\n</span><span style=\"color:#c0c5ce;\">        </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> color = </span><span style=\"color:#b48ead;\">if</span><span style=\"color:#c0c5ce;\"> result.distance &lt; </span><span style=\"color:#d08770;\">0.25 </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">            Color::Green\n</span><span style=\"color:#c0c5ce;\">        } </span><span style=\"color:#b48ead;\">else if</span><span style=\"color:#c0c5ce;\"> result.distance &lt; </span><span style=\"color:#d08770;\">0.75 </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">            Color::Yellow\n</span><span style=\"color:#c0c5ce;\">        } </span><span style=\"color:#b48ead;\">else </span><span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">            Color::Red\n</span><span style=\"color:#c0c5ce;\">        };\n</span><span style=\"color:#c0c5ce;\">        row.</span><span style=\"color:#96b5b4;\">add_cell</span><span style=\"color:#c0c5ce;\">(Cell::new(result.distance).</span><span style=\"color:#96b5b4;\">fg</span><span style=\"color:#c0c5ce;\">(color))\n</span><span style=\"color:#c0c5ce;\">            .</span><span style=\"color:#96b5b4;\">add_cell</span><span style=\"color:#c0c5ce;\">(Cell::new(result.record.</span><span style=\"color:#96b5b4;\">body</span><span style=\"color:#c0c5ce;\">()[</span><span style=\"color:#d08770;\">0</span><span style=\"color:#c0c5ce;\">..</span><span style=\"color:#d08770;\">50</span><span style=\"color:#c0c5ce;\">].</span><span style=\"color:#96b5b4;\">to_string</span><span style=\"color:#c0c5ce;\">() + &quot;</span><span style=\"color:#a3be8c;\">...</span><span style=\"color:#c0c5ce;\">&quot;));\n</span><span style=\"color:#c0c5ce;\">        table.</span><span style=\"color:#96b5b4;\">add_row</span><span style=\"color:#c0c5ce;\">(row);\n</span><span style=\"color:#c0c5ce;\">    }\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    println!(&quot;</span><span style=\"color:#d08770;\">{}</span><span style=\"color:#c0c5ce;\">&quot;, table);\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "rag-improvements",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#rag-improvements",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "RAG improvements",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "RAG (Retrieval-Augmented Generation) is a powerful tool for generating text with up-to-date or proprietary information. Retrieval-augmented generation generally follows the following steps:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ol::TAG_NAME,
                        namespace: dioxus_elements::ol::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Gather context from some local files, your database, or web data. In kalosm, you can retrieve data from any source that implements "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/trait.IntoDocument.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "" },
                                {
                                    dioxus_core :: TemplateNode :: Element
                                    {
                                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                        [], children : &
                                        [dioxus_core :: TemplateNode :: Text
                                        { text : "IntoDocument" }],
                                    }
                                }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : " or " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/trait.IntoDocuments.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "" },
                                {
                                    dioxus_core :: TemplateNode :: Element
                                    {
                                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                        [], children : &
                                        [dioxus_core :: TemplateNode :: Text
                                        { text : "IntoDocuments" }],
                                    }
                                }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        { text : ". You can gather your sources from " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/struct.DocumentFolder.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "local documents" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : ", a " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/struct.SearchQuery.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "search term" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : ", " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/struct.Url.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "specific web page" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : ", an " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/struct.RssFeed.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "RSS feed" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        { text : ", or even a " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/enum.Page.html#method.crawl",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "custom web crawler" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text { text : "." }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Insert that context into a searchable database. Kalosm includes a "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: a :: TAG_NAME,
                                namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                                [dioxus_core :: TemplateAttribute :: Static
                                {
                                    name : dioxus_elements :: a :: href.0, namespace :
                                    dioxus_elements :: a :: href.1, value :
                                    "https://docs.rs/kalosm/0.2.0/kalosm/language/struct.VectorDB.html",
                                }], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "vector database" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " that can be used to store and retrieve similar documents."
                        }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Vector databases use an embedding model which generates a vector for a chunk of text (typically smaller than the entire document). The vector is then stored in the database. When you want to retrieve similar documents, you can embed a query and search for similar vectors in the database."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "The vectors represent the meaning of the text, so you can search for similar documents based on the meaning of the text, not just the words used."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ol::TAG_NAME,
                        namespace: dioxus_elements::ol::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Use the context to generate text. You can find text similar to the question or a search generated by the LLM and then generate a response based on the context."
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text: "In this release, we've made several improvements to RAG! (",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://github.com/floneum/floneum/pull/126",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text { text: "#126" }],
                                }
                            },
                            dioxus_core::TemplateNode::Text { text: ")" },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "improved-chunking-strategies",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#improved-chunking-strategies",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Improved Chunking Strategies",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "When you insert a document into a vector database, it needs to be split into smaller chunks before the text is embedded. The chunks you choose can have a significant impact on the performance of the results you get from the vector database. In this release, we've added two new chunking strategies to the vector database:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Hypothetical Questions",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Instead of generating embeddings based on the content of the document, this chunking strategy generates embeddings based on hypothetical questions generated about the document. This can be extremely useful when building a chatbot that needs to find context that is relevant to a question."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "For example, if you have a document about the history of the United States, you can generate hypothetical questions like \"What is the capital of the United States?\" and \"Who was the first president of the United States?\" and then generate embeddings based on those questions."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Then if you query the vector database with a question like \"Who was the leader of the US?\" you can find the document about the history of the United States."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: p :: TAG_NAME,
                        namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Notice that the question \"Who was the leader of the US?\" doesn't contain many of the same words as the hypothetical questions, but it does convey a similar meaning, so the vector database can still find the relevant document."
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::li::TAG_NAME,
                                namespace: dioxus_elements::li::NAME_SPACE,
                                attrs: &[],
                                children: &[dioxus_core::TemplateNode::Text { text: "Summaries" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "This chunking strategy generates embeddings based on the summary of the document. This can be useful when you have a large document and you want to find similar documents based on the main points of the document."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Generating embeddings based on the summary of the document can create better embeddings that contain more information about the document than embeddings that only contain one small chunk of the document."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h3::TAG_NAME,
                        namespace: dioxus_elements::h3::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h3::id.0,
                            namespace: dioxus_elements::h3::id.1,
                            value: "incremental-indexing",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#incremental-indexing",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Incremental Indexing",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "In addition to the new chunking strategies, we've also added support for incremental indexing. This means you can add new documents to the vector database without having to recreate the entire database. This can be extremely useful when you have a large database or you have constantly updating context you want to provide to your LLM."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "Kalosm's Vector database is now backed by " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/meilisearch/arroy",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "arroy" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    ", a space-efficient and incrementally indexed vector database backed by MeiliSearch!"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "performance-improvements",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#performance-improvements",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Performance Improvements",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "The llama implementation has been rewritten and optimized for better performance and modularity. The new implementation is now 7-25% faster than the previous version. In future releases, we plan to add support for fine tuning models and training new heads for existing models. ("
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/pull/122",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "#122" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ")" }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Language models like Llama and Phi output probabilities for each token in the vocabulary. To generate text you need to sample from the probability distribution. Sampling from the probability distribution can be slow, especially with large vocabularies. Kalosm 0.2 uses an optimization introduced in "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/KerfuffleV2/llm-samplers/pull/9",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "llm-samplers" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " to only sample top 512 tokens. This optimization can make sampling up to 2x faster. ("
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/pull/123",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "#123" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ")" }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Large sections of text that are static within a constraint in structured generation is now loaded in a batch which can significantly speed up the process."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "For example if you have the constraints:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> constraints = RegexParser::new(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#c0c5ce;\">#</span><span style=\"color:#a3be8c;\">&quot;(The title of the book is [a-z]{2,15}\\n)*</span><span style=\"color:#c0c5ce;\">&quot;#).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "The text \"The title of the book is \" will be loaded in a batch instead of one token at a time. Batched loading has been restored in constrained generation. ("
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/pull/131",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "#131" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : ")" }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "new-models",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#new-models",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text { text: "New Models" }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Kalosm 0.2 adds support for several new models, including:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[
                                        {
                                            dioxus_core::TemplateNode::Element {
                                                tag: dioxus_elements::elements::strong::TAG_NAME,
                                                namespace: dioxus_elements::strong::NAME_SPACE,
                                                attrs: &[],
                                                children: &[dioxus_core::TemplateNode::Text {
                                                    text: "Dolphin Phi v2",
                                                }],
                                            }
                                        },
                                        dioxus_core::TemplateNode::Text {
                                            text: " A tiny chat model",
                                        },
                                    ],
                                }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: strong :: TAG_NAME,
                                namespace : dioxus_elements :: strong :: NAME_SPACE, attrs :
                                & [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "Solar-11b Models" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " A set of models for chat, text, and code generation"
                        }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [{
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: strong :: TAG_NAME,
                                namespace : dioxus_elements :: strong :: NAME_SPACE, attrs :
                                & [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "Tiny Llama 1.0" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            " A tiny set of models for chat, and text text generation"
                        }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "full-changelog",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#full-changelog",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Full Changelog",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "For a detailed list of changes between v0.1.0 and v0.2.0, please see the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/compare/v0.2.0...v0.2.0-kalosm",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text
                        { text : "full changelog" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : "." }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "I hope you enjoy using Kalosm v0.2.0! Your feedback is invaluable to us, so please don't hesitate to share your thoughts and report any issues you encounter."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "whats-next",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#whats-next",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "What's next?",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "In the next release, we plan to add support for fine tuning models and training new heads for existing models. We also plan to continue improving the performance of the language models and adding support for more models."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "If any of those features sound interesting or you want to propose a new feature, consider contributing on "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/tree/main/interfaces/kalosm",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Github" }],
                    }
                }, dioxus_core :: TemplateNode :: Text { text : "." }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            dioxus_core::TemplateNode::Text {
                                text:
                                    "If you are interested in building an application with Kalosm, ",
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::a::TAG_NAME,
                                    namespace: dioxus_elements::a::NAME_SPACE,
                                    attrs: &[dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "https://discord.gg/dQdmhuB8q5",
                                    }],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "join the Discord",
                                    }],
                                }
                            },
                            dioxus_core::TemplateNode::Text {
                                text: " and get involved with the community!",
                            },
                        ],
                    }
                },
            ],
            node_paths: &[],
            attr_paths: &[],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(None, ___TEMPLATE, Box::new([]), Box::new([]));
            __vnodes
        }
    });
    dioxus_core::Element::Ok({
        #[doc(hidden)]
        static ___TEMPLATE: dioxus_core::Template = dioxus_core::Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", 128usize),
            roots: &[
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h1::TAG_NAME,
                        namespace: dioxus_elements::h1::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h1::id.0,
                            namespace: dioxus_elements::h1::id.1,
                            value: "structured-generation-visualized",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#structured-generation-visualized",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Structured Generation Visualized",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 0usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Structured Generation Visualized",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "First some background:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ul::TAG_NAME,
                        namespace: dioxus_elements::ul::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: "Tokens" }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text { text: "LLMs" }],
                                }
                            },
                            {
                                dioxus_core::TemplateNode::Element {
                                    tag: dioxus_elements::elements::li::TAG_NAME,
                                    namespace: dioxus_elements::li::NAME_SPACE,
                                    attrs: &[],
                                    children: &[dioxus_core::TemplateNode::Text {
                                        text: "Grammars/Incremental parsing",
                                    }],
                                }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "Structured Generation Visualized",
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Text is a universal format for data. Data in text form is all over the web. Communication happens in JSON, code is written in . Because LLMs are trained on a giant corpus of web text, they can generally understand and write text in a machine readable format like JSON."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "I want 100 different random characters with reasonable names, descriptions, and ages. Instead of creating each character individually, what if we ask a small LLM to generate JSON for each character?"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "We need JSON in this format:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">name</span><span style=\"color:#c0c5ce;\">&quot;: string,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">description</span><span style=\"color:#c0c5ce;\">&quot;: string,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">metadata</span><span style=\"color:#c0c5ce;\">&quot;: {\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">age</span><span style=\"color:#c0c5ce;\">&quot;: number,\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">height</span><span style=\"color:#c0c5ce;\">&quot;: number (cm),\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">weight</span><span style=\"color:#c0c5ce;\">&quot;: number (kg),\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">hair_color</span><span style=\"color:#c0c5ce;\">&quot;: string,\n</span><span style=\"color:#c0c5ce;\">        &quot;</span><span style=\"color:#a3be8c;\">eye_color</span><span style=\"color:#c0c5ce;\">&quot;: string,\n</span><span style=\"color:#c0c5ce;\">    }\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                { text : "We can use the " },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: a :: TAG_NAME,
                        namespace : dioxus_elements :: a :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: a :: href.0, namespace :
                            dioxus_elements :: a :: href.1, value :
                            "https://github.com/floneum/floneum/tree/main/interfaces/kalosm",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Kalosm" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " library to generate text with the phi-3-mini-4k-instruct model. We will use the "
                },
                {
                    dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: code :: TAG_NAME,
                        namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Task" }],
                    }
                }, dioxus_core :: TemplateNode :: Text
                {
                    text :
                    " struct to create a task that streams unstructured text into stdout:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#65737e;\">// Cargo.toml\n</span><span style=\"color:#65737e;\">// [dependencies]\n</span><span style=\"color:#65737e;\">// kalosm = { version = &quot;0.3&quot;, features = [&quot;language&quot;, &quot;metal&quot;] }\n</span><span style=\"color:#65737e;\">// tokio = { version = &quot;1.37.0&quot;, features = [&quot;full&quot;] }\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Create a new model. We are using the Phi-3 model which is small and focused on reasoning tasks.\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = Llama::builder()\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">with_source</span><span style=\"color:#c0c5ce;\">(LlamaSource::phi_3_mini_4k_instruct())\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">()\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Create a task that generates text for a character\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> task = Task::builder(&quot;</span><span style=\"color:#a3be8c;\">You generate data in a JSON format</span><span style=\"color:#c0c5ce;\">&quot;).</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Run the task\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> result = task.</span><span style=\"color:#96b5b4;\">run</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#c0c5ce;\">#</span><span style=\"color:#a3be8c;\">&quot;Generate a character with this format: { &quot;name&quot;: string, &quot;description&quot;: string, &quot;metadata&quot;: { &quot;age&quot;: number, &quot;height_cm&quot;: number, &quot;weight_kg&quot;: number, &quot;hair_color&quot;: string, &quot;eye_color&quot;: string } }</span><span style=\"color:#c0c5ce;\">&quot;#, &amp;model);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Stream the text into stdout\n</span><span style=\"color:#c0c5ce;\">    result.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Running the program sometimes generates valid JSON with reasonable data:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">  &quot;</span><span style=\"color:#a3be8c;\">name</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">Michael Thompson</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">  &quot;</span><span style=\"color:#a3be8c;\">description</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">A dedicated software developer with a passion for coding and innovation. Known amongst friends to be an avid reader, particularly of science fiction novels.</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">  &quot;</span><span style=\"color:#a3be8c;\">metadata</span><span style=\"color:#c0c5ce;\">&quot;: {\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">age</span><span style=\"color:#c0c5ce;\">&quot;: </span><span style=\"color:#d08770;\">29</span><span style=\"color:#c0c5ce;\">,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">height_cm</span><span style=\"color:#c0c5ce;\">&quot;: </span><span style=\"color:#d08770;\">180</span><span style=\"color:#c0c5ce;\">,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">weight_kg</span><span style=\"color:#c0c5ce;\">&quot;: </span><span style=\"color:#d08770;\">75</span><span style=\"color:#c0c5ce;\">,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">hair_color</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">dark brown</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">eye_color</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">blue</span><span style=\"color:#c0c5ce;\">&quot;\n</span><span style=\"color:#c0c5ce;\">  }\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[dioxus_core::TemplateNode::Text {
                            text: "But other times, it generates nonsense:",
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">{\n</span><span style=\"color:#c0c5ce;\">   &quot;</span><span style=\"color:#a3be8c;\">name</span><span style=\"color:#c0c5ce;\">&quot;:&quot;</span><span style=\"color:#a3be8c;\">Emily Thompson</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">description</span><span style=\"color:#c0c5ce;\">&quot;:&quot;</span><span style=\"color:#a3be8c;\">A talented graphic designer with an eye-catching style. Emily is known to be creative, friendly and dedicated at her job.</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">     &quot;</span><span style=\"color:#a3be8c;\">metadata</span><span style=\"color:#c0c5ce;\">&quot;: { \n</span><span style=\"color:#c0c5ce;\">         &quot;</span><span style=\"color:#a3be8c;\">age</span><span style=\"color:#c0c5ce;\">&quot;:</span><span style=\"color:#d08770;\">28</span><span style=\"color:#c0c5ce;\">,\n</span><span style=\"color:#c0c5ce;\">          &quot;</span><span style=\"color:#a3be8c;\">height</span><span style=\"color:#c0c5ce;\">&quot;:</span><span style=\"color:#d08770;\">165</span><span style=\"color:#c0c5ce;\">,&quot;</span><span style=\"color:#a3be8c;\">cm</span><span style=\"color:#c0c5ce;\">&quot; :</span><span style=\"color:#d08770;\">94</span><span style=\"color:#c0c5ce;\">,\n</span><span style=\"color:#c0c5ce;\">           &quot;</span><span style=\"color:#a3be8c;\">weight</span><span style=\"color:#c0c5ce;\">&quot;:&quot;</span><span style=\"color:#a3be8c;\">60</span><span style=\"color:#c0c5ce;\">&quot;,&quot;</span><span style=\"color:#a3be8c;\">kg</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">            &quot;</span><span style=\"color:#a3be8c;\">hair_color</span><span style=\"color:#c0c5ce;\">&quot;:&quot;</span><span style=\"color:#a3be8c;\">blonde</span><span style=\"color:#c0c5ce;\">&quot;,\n</span><span style=\"color:#c0c5ce;\">             &quot;</span><span style=\"color:#a3be8c;\">eye_color</span><span style=\"color:#c0c5ce;\">&quot;:&quot;</span><span style=\"color:#a3be8c;\">blue</span><span style=\"color:#c0c5ce;\">&quot; }  \n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Instead of just telling the LLM about the format we want, what if we force it to generate text that conforms to the format?"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "token-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#token-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Token Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "First, lets dive into a bit of background on LLMs. Large language models are trained on a massive corpus of text. Instead of reading characters, or words in a sentence, the LLM is trained to read chunks of text called tokens. On average each token is about 2/3 of a word, but depending on the word it could be the entire word or a single character."
                }],
            }
                },
                dioxus_core::TemplateNode::Dynamic { id: 0usize },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "To generate text, LLMs assign a probability to each token and picks a token with a high probability. Picking a token from the list of probabilities is called sampling."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "incremental-parsing",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#incremental-parsing",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Incremental Parsing",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Determining exactly what sequences are valid can be more difficult than it first appears. We need a parser that:"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::ol::TAG_NAME,
                        namespace: dioxus_elements::ol::NAME_SPACE,
                        attrs: &[],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Incrementally parses new text in a way we can roll back. If the current tokens are "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text
                                { text : "{\"age\":" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        { text : " we need to be able to try adding " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "a" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        { text : " and if it fails, roll back to " },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "{\"age\"" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            ". It shouldn't need to re-parse the entire string for every one of the "
                        },
                        {
                            dioxus_core :: TemplateNode :: Element
                            {
                                tag : dioxus_elements :: elements :: code :: TAG_NAME,
                                namespace : dioxus_elements :: code :: NAME_SPACE, attrs : &
                                [], children : &
                                [dioxus_core :: TemplateNode :: Text { text : "128,000" }],
                            }
                        }, dioxus_core :: TemplateNode :: Text
                        { text : " possible new tokens." }],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: li :: TAG_NAME,
                        namespace : dioxus_elements :: li :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Fails fast. If a new token is invalid, we need to know that immediately. We can't batch up several new tokens and try to parse them all at once."
                        }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "It turns out Regular Expressions are very well suited for this task. A regular expression can be represented as a finite state machine. Each state in the machine can be stored and restored easily."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#65737e;\">// Cargo.toml\n</span><span style=\"color:#65737e;\">// [dependencies]\n</span><span style=\"color:#65737e;\">// kalosm = { version = &quot;0.3&quot;, features = [&quot;language&quot;, &quot;metal&quot;] }\n</span><span style=\"color:#65737e;\">// tokio = { version = &quot;1.37.0&quot;, features = [&quot;full&quot;] }\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#b48ead;\">use </span><span style=\"color:#c0c5ce;\">kalosm::language::*;\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">#[</span><span style=\"color:#bf616a;\">tokio</span><span style=\"color:#c0c5ce;\">::</span><span style=\"color:#bf616a;\">main</span><span style=\"color:#c0c5ce;\">]\n</span><span style=\"color:#c0c5ce;\">async </span><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Create a new model. We are using the Phi-3 model which is small and focused on reasoning tasks.\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> model = Llama::phi_3()\n</span><span style=\"color:#c0c5ce;\">        .await\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Create a constraint that checks if the generated text is valid JSON\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> constraint = RegexParser::new(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#c0c5ce;\">#</span><span style=\"color:#a3be8c;\">&quot;\\{ &quot;name&quot;: &quot;[A-Z][a-z]{1,10} [A-Z][a-z]{1,10}&quot;, &quot;description&quot;: &quot;[ A-Za-z]+&quot;, &quot;metadata&quot;: \\{ &quot;age&quot;: \\d{1,2}, &quot;height_cm&quot;: \\d{1,3}, &quot;weight_kg&quot;: \\d{1,3}, &quot;hair_color&quot;: &quot;[A-Z][a-z]+&quot;, &quot;eye_color&quot;: &quot;[A-Z][a-z]+&quot; \\} \\}</span><span style=\"color:#c0c5ce;\">&quot;#).</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Create a task that generates text for a character\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> task = Task::builder(&quot;</span><span style=\"color:#a3be8c;\">You generate data in a JSON format</span><span style=\"color:#c0c5ce;\">&quot;)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">with_constraints</span><span style=\"color:#c0c5ce;\">(constraint)\n</span><span style=\"color:#c0c5ce;\">        .</span><span style=\"color:#96b5b4;\">build</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Run the task\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#b48ead;\">let</span><span style=\"color:#c0c5ce;\"> result = task.</span><span style=\"color:#96b5b4;\">run</span><span style=\"color:#c0c5ce;\">(</span><span style=\"color:#b48ead;\">r</span><span style=\"color:#c0c5ce;\">#</span><span style=\"color:#a3be8c;\">&quot;Generate a character with this format: { &quot;name&quot;: string, &quot;description&quot;: string, &quot;metadata&quot;: { &quot;age&quot;: number, &quot;height_cm&quot;: number, &quot;weight_kg&quot;: number, &quot;hair_color&quot;: string, &quot;eye_color&quot;: string } }</span><span style=\"color:#c0c5ce;\">&quot;#, &amp;model);\n</span><span style=\"color:#c0c5ce;\">\n</span><span style=\"color:#c0c5ce;\">    </span><span style=\"color:#65737e;\">// Stream the text into stdout\n</span><span style=\"color:#c0c5ce;\">    result.</span><span style=\"color:#96b5b4;\">to_std_out</span><span style=\"color:#c0c5ce;\">().await.</span><span style=\"color:#96b5b4;\">unwrap</span><span style=\"color:#c0c5ce;\">();\n</span><span style=\"color:#c0c5ce;\">}\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::div::TAG_NAME,
                        namespace: dioxus_elements::div::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::div::style.0,
                            namespace: dioxus_elements::div::style.1,
                            value: "position: relative;",
                        }],
                        children: &[
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: div :: TAG_NAME,
                        namespace : dioxus_elements :: div :: NAME_SPACE, attrs : &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: div :: dangerous_inner_html.0,
                            namespace : dioxus_elements :: div ::
                            dangerous_inner_html.1, value :
                            "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#c0c5ce;\">{ &quot;</span><span style=\"color:#a3be8c;\">name</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">Evelyn Archer</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">description</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">A cunning and resourceful detective with a sharp eye for detail who has solved numerous complex cases in the bustling city of New York during her prime years as an investigator at midlife crisis stage when she starts to question life choices leading up until now that age is just another number</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">metadata</span><span style=\"color:#c0c5ce;\">&quot;: { &quot;</span><span style=\"color:#a3be8c;\">age</span><span style=\"color:#c0c5ce;\">&quot;: </span><span style=\"color:#d08770;\">45</span><span style=\"color:#c0c5ce;\">, &quot;</span><span style=\"color:#a3be8c;\">height_cm</span><span style=\"color:#c0c5ce;\">&quot;: </span><span style=\"color:#d08770;\">168</span><span style=\"color:#c0c5ce;\">, &quot;</span><span style=\"color:#a3be8c;\">weight_kg</span><span style=\"color:#c0c5ce;\">&quot;: </span><span style=\"color:#d08770;\">70</span><span style=\"color:#c0c5ce;\">, &quot;</span><span style=\"color:#a3be8c;\">hair_color</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">Auburn</span><span style=\"color:#c0c5ce;\">&quot;, &quot;</span><span style=\"color:#a3be8c;\">eye_color</span><span style=\"color:#c0c5ce;\">&quot;: &quot;</span><span style=\"color:#a3be8c;\">Hazel</span><span style=\"color:#c0c5ce;\">&quot; } }\n</span></pre>\n",
                        }], children : & [],
                    }
                            },
                            {
                                dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: button :: TAG_NAME,
                        namespace : dioxus_elements :: button :: NAME_SPACE, attrs :
                        &
                        [dioxus_core :: TemplateAttribute :: Static
                        {
                            name : dioxus_elements :: button :: style.0, namespace :
                            dioxus_elements :: button :: style.1, value :
                            "position: absolute; top: 0; right: 0; background: rgba(0, 0, 0, 0.75); color: white; border: 1px solid white; padding: 0.25em;",
                        }, dioxus_core :: TemplateAttribute :: Static
                        {
                            name : "onclick", namespace : None, value :
                            "navigator.clipboard.writeText(this.previousElementSibling.innerText)",
                        }], children : &
                        [dioxus_core :: TemplateNode :: Text { text : "Copy" }],
                    }
                            },
                        ],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "constrained-augmented-sampling",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#constrained-augmented-sampling",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Constrained Augmented Sampling",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Instead of just picking a token with the highest probability, we can filter the probabilities to only include tokens that are valid for our format. Even if you choose completely random options from the list of probabilities, the LLM will still generate text that conforms to the format."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "From the LLM perspective, it can only \"choose\" from the valid tokens. Something like this:"
                }],
            }
                },
                dioxus_core::TemplateNode::Dynamic { id: 1usize },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "accelerated-structured-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#accelerated-structured-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Accelerated Structured Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "You may have noticed that some of the choices the LLM makes only have one valid next token. We don't actually need to run the LLM at all for these choices. Instead, we can just choose the next token directly. Now the LLM only chooses valid tokens where there is an interesting choice to make:"
                }],
            }
                },
                dioxus_core::TemplateNode::Dynamic { id: 2usize },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Instead of choosing each one of the ~16 tokens, we only need to choose between the ~4 important tokens."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::h2::TAG_NAME,
                        namespace: dioxus_elements::h2::NAME_SPACE,
                        attrs: &[dioxus_core::TemplateAttribute::Static {
                            name: dioxus_elements::h2::id.0,
                            namespace: dioxus_elements::h2::id.1,
                            value: "sampler-aware-structured-generation",
                        }],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::a::TAG_NAME,
                                namespace: dioxus_elements::a::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::href.0,
                                        namespace: dioxus_elements::a::href.1,
                                        value: "#sampler-aware-structured-generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::a::class.0,
                                        namespace: dioxus_elements::a::class.1,
                                        value: "header",
                                    },
                                ],
                                children: &[dioxus_core::TemplateNode::Text {
                                    text: "Sampler Aware Structured Generation",
                                }],
                            }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "Right now, we are running the parser for every single one of the 128,000 tokens every time we add a new token to the sequence. This is very inefficient. Instead, we can take advantage of the structure of the sampler to only run the parser for the tokens that could actually be sampled."
                }],
            }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "There are generally a few steps between the token probabilities after constraints and the token that gets chosen. Each of those steps that modify the probabilities of the tokens is called a sampler. One common sampler is the top-k sampler which only samples the top k tokens with the highest probability. Here is what that could look like if we only keep the top 2 tokens (k=2):"
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 1usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Top-k Sampling",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::blockquote::TAG_NAME,
                        namespace: dioxus_elements::blockquote::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core :: TemplateNode :: Element
                    {
                        tag : dioxus_elements :: elements :: p :: TAG_NAME,
                        namespace : dioxus_elements :: p :: NAME_SPACE, attrs : &
                        [], children : &
                        [dioxus_core :: TemplateNode :: Text
                        {
                            text :
                            "Generally k is larger, but much smaller than the number of tokens in the model. The default in kalosm if 64."
                        }],
                    }
                        }],
                    }
                },
                {
                    dioxus_core :: TemplateNode :: Element
            {
                tag : dioxus_elements :: elements :: p :: TAG_NAME, namespace
                : dioxus_elements :: p :: NAME_SPACE, attrs : & [], children :
                &
                [dioxus_core :: TemplateNode :: Text
                {
                    text :
                    "We can combine the top-k sampler with constrained generation by only looking for the top k valid tokens after the constraints have been applied. Once we have the most probable k tokens, we can stop running the constraints against tokens at all. Since the LLM knows about the constraints, the valid tokens tend to have a very high probability which means we can skip parsing the majority of the 128,000 tokens."
                }],
            }
                },
                {
                    dioxus_core::TemplateNode::Element {
                        tag: dioxus_elements::elements::p::TAG_NAME,
                        namespace: dioxus_elements::p::NAME_SPACE,
                        attrs: &[],
                        children: &[{
                            dioxus_core::TemplateNode::Element {
                                tag: dioxus_elements::elements::img::TAG_NAME,
                                namespace: dioxus_elements::img::NAME_SPACE,
                                attrs: &[
                                    dioxus_core::TemplateAttribute::Dynamic { id: 2usize },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::alt.0,
                                        namespace: dioxus_elements::img::alt.1,
                                        value: "Top-K Accelerated Structured Generation",
                                    },
                                    dioxus_core::TemplateAttribute::Static {
                                        name: dioxus_elements::img::title.0,
                                        namespace: dioxus_elements::img::title.1,
                                        value: "",
                                    },
                                ],
                                children: &[],
                            }
                        }],
                    }
                },
            ],
            node_paths: &[&[18u8], &[29u8], &[32u8]],
            attr_paths: &[&[1u8, 0u8], &[37u8, 0u8], &[40u8, 0u8]],
        };
        {
            #[allow(clippy::let_and_return)]
            let __vnodes = dioxus_core::VNode::new(
                None,
                ___TEMPLATE,
                Box::new([
                    dioxus_core::DynamicNode::Component({
                        use dioxus_core::prelude::Properties;
                        let __comp = ({ fc_to_builder(TokenizationVisualization).build() })
                            .into_vcomponent(
                                TokenizationVisualization,
                                "TokenizationVisualization",
                            );
                        __comp
                    }),
                    dioxus_core::DynamicNode::Component({
                        use dioxus_core::prelude::Properties;
                        let __comp = ({ fc_to_builder(StructuredGenerationVisualization).build() })
                            .into_vcomponent(
                                StructuredGenerationVisualization,
                                "StructuredGenerationVisualization",
                            );
                        __comp
                    }),
                    dioxus_core::DynamicNode::Component({
                        use dioxus_core::prelude::Properties;
                        let __comp = ({
                            fc_to_builder(StructuredGenerationAcceleratedVisualization).build()
                        })
                        .into_vcomponent(
                            StructuredGenerationAcceleratedVisualization,
                            "StructuredGenerationAcceleratedVisualization",
                        );
                        __comp
                    }),
                ]),
                Box::new([
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file(
                                "./public/assets/structured_generation_visualized.png"
                            )),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file("./public/assets/top_k_sampling.png")),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                    Box::new([{
                        dioxus_core::Attribute::new(
                            dioxus_elements::img::src.0,
                            manganis::mg!(file(
                                "./public/assets/top_k_accelerated_structured_generation.png"
                            )),
                            dioxus_elements::img::src.1,
                            dioxus_elements::img::src.2,
                        )
                    }]),
                ]),
            );
            __vnodes
        }
    })
}
