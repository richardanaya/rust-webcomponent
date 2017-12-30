#![recursion_limit="256"]
#[macro_use]
extern crate stdweb;

// This function allows us to define a new html element hooked up to the static members of a type
fn define_web_component<T:WebComponent + 'static>(_:T) {

    // we need to use the array of attribute names we should be observing
    // and pass them in as a joined string since giving arrays to stdweb
    // isn't possible or expensive
    let observed_attributes = T::get_observable_attributes().join(":");

    js! {
        // use a global variable that allows us to give a context for what element
        // is currently handling an event
        window.currentElement = null;

        // create a generated custom element
        class GeneratedCustomElement extends HTMLElement {
          static get observedAttributes() {return (@{observed_attributes}).split(":"); }

          constructor() {
              super();
              window.currentElement = this;
              (@{T::constructor})();
              window.currentElement = null;
          }

          connectedCallback() {
            window.currentElement = this;
            (@{T::connected})();
            window.currentElement = null;
          }

          disconnectedCallback() {
            window.currentElement = this;
            (@{T::disconnected})();
            window.currentElement = null;
          }

          attributeChangedCallback(attributeName, oldValue, newValue) {
            window.currentElement = this;
            (@{T::attribute_changed})(attributeName,oldValue||"",newValue||"");
            window.currentElement = null;
          }
        }

        // tell the dom to associate it with an html tag name
        customElements.define(@{T::get_element_name()}, GeneratedCustomElement);
    }
}

fn log(msg:&str) {
    js! {
        console.log(@{msg});
    }
}

fn get_attribute(attr_name:&str) -> String{
    let result = js! {
        return window.currentElement.getAttribute(@{attr_name})||"";
    };
    result.as_str().unwrap().to_string()
}

fn alert(msg:&str) {
    js! {
        alert(@{msg});
    }
}

fn set_inner_html(html:&str){
    js! {
        window.currentElement.innerHTML = @{html};
    }
}

fn set_child_inner_html(target:&str,html:&str){
    js! {
        window.currentElement.querySelector(@{target}).innerHTML = @{html};
    }
}

fn add_event_listener(event_type:&str,handler:fn()->()){
    js! {
        window.currentElement.addEventListener(@{event_type}, () => {
            (@{handler})();
        })
    }
}

trait WebComponent {
    fn get_element_name() -> &'static str {""}
    fn get_observable_attributes() -> Vec<&'static str> {vec![]}
    fn constructor(){}
    fn connected(){}
    fn disconnected(){}
    fn attribute_changed(_attribute_name:String,_old_value:String,_new_value:String){}
}

struct HelloWorld;

// this is an example web component that does some stuff on web component lifecycles
// since there are defaul definitions in the trait, these aren't really all necessary
impl WebComponent for HelloWorld {
    fn get_element_name() -> &'static str {"hello-world"}

    fn get_observable_attributes() -> Vec<&'static str> {vec!["greeting","name"]}

    fn constructor(){
        set_inner_html(r#"
            <style>
                hello-world button {
                    border: solid 1px black;
                    border-radius: 5px;
                    padding: 5px;
                    font-family: arial;
                }
            </style>
            <button>Hello World!</button>
          "#);
         add_event_listener("click",||{
             alert("Surprise!");
         })
    }

    fn connected(){
        log("connected");
    }

    fn disconnected(){
        log("disconnected");
    }

    fn attribute_changed(attribute_name:String,_old_value:String,new_value:String){
        if attribute_name == "greeting" {
            let name_attr = get_attribute("name");
            let name = match name_attr.len() {
                0 => "World",
                _ => &name_attr
            };
            set_child_inner_html("button",&format!("{} {}!",new_value,name));
        } else if attribute_name == "name" {
            let greeting_attr = get_attribute("greeting");
            let greeting = match greeting_attr.len() {
                0 => "Hello",
                _ => &greeting_attr
            };
            set_child_inner_html("button",&format!("{} {}!",new_value,greeting));
        }
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    define_web_component(HelloWorld);

    // inject some dom into the body to show it off!
    js! {
        document.body.innerHTML = @{r#"
            <hello-world></hello-world>
            <hello-world greeting="Hola" name="Mundo"></hello-world>
        "#};
    }

    // keep std event going
    stdweb::event_loop();
}
