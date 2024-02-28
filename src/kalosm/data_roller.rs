use dioxus::prelude::*;

pub fn DataAnimation() -> Element {
    let items = ["Data", "Code", "Diary", "Conversations", "History"];
    let mut iter = items.iter();
    rsx! {
        ul {
            for (i, item) in iter.enumerate() {
                li {
                    class: "slideIn-item-{i+1}",
                    opacity: "0",
                    position: "absolute",
                    {item}
                }
            }
        }
        style {
            ".slideIn-item-1 {{
                animation: slideIn 15s 0s forwards cubic-bezier(0.1, 0.67, 0.29, 0.98) infinite;
            }}
            .slideIn-item-2 {{
                /* Start with 3 seconds delay */
                animation: slideIn 15s 3s forwards cubic-bezier(0.1, 0.67, 0.29, 0.98) infinite;
            }}
            .slideIn-item-3 {{
                /* Start with 6 seconds delay */
                animation: slideIn 15s 6s forwards cubic-bezier(0.1, 0.67, 0.29, 0.98) infinite;
            }}
            .slideIn-item-4 {{
                /* Start with 9 seconds delay */
                animation: slideIn 15s 9s forwards cubic-bezier(0.1, 0.67, 0.29, 0.98) infinite;
            }}
            .slideIn-item-5 {{
                /* Start with 12 seconds delay */
                animation: slideIn 15s 12s forwards cubic-bezier(0.1, 0.67, 0.29, 0.98) infinite;
            }}
            
            @keyframes slideIn {{
                0% {{
                    transform: translate3d(50%, 0, 0);
                    opacity: 0;
                }}
                1.00% {{
                    transform: translate3d(0, 0, 0);
                    opacity: 1;
                }}
                10.00% {{
                    transform: translate3d(0, 0, 0);
                    opacity: 1;
                }}
                24.00% {{
                    transform: translate3d(-50%, 0, 0);
                    opacity: 0;
                }}
                25.00% {{
                    transform: translate3d(50%, 0, 0);
                    opacity: 0;
                }}
                100% {{
                    transform: translate3d(50%, 0, 0);
                    opacity: 0;
                }}
            }}"
        }
    }
}
