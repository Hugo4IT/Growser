# Growser
A web browser made in Godot, just because I can.

It currently downloads a page from a URL, parses the HTML with [html5ever](https://github.com/servo/html5ever) and generates a full node tree (sadly the order of child nodes in a parent is random) of panels, HBox-/VBox- Containers and Labels. It has no understanding of style or script, but it recognizes external source code and where to find it.

The browser functionality is implemented in Rust with GDNative, while the browser frontend (search bar, buttons, history, etc) is handled with GDScript.

## Screenshot

![image](https://i.ibb.co/5cwYMZV/Screenshot-20220117-111942.png)
