extends Control

const Html = preload("res://Web/Scripts/GDNative/Html.gdns")

onready var page_holder: ScrollContainer = get_node("Margins/Components/Page/ScrollContainer")
onready var uri_input: LineEdit = get_node("Margins/Components/Toolbar/URI")
onready var go_button: Button = get_node("Margins/Components/Toolbar/Go")

var html: Html
var http: HTTPRequest

func _ready() -> void:
	http = HTTPRequest.new()
	http.connect("request_completed", self, "_response")
	add_child(http)
	
	go_button.connect("pressed", self, "_search")
	uri_input.text = BrowserState.last_page
	
	html = Html.new()
	page_holder.add_child(html)

func _exit_tree() -> void:
	BrowserState.save()

# go_button->pressed
func _search() -> void:
	# GET / HTTP/3
	# Host: hugo4it.com
	# User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:96.0) Gecko/20100101 Firefox/96.0
	# Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8
	# Accept-Language: en-US,en;q=0.5
	# Accept-Encoding: gzip, deflate, br
	# DNT: 1
	# Alt-Used: hugo4it.com
	# Connection: keep-alive
	# Upgrade-Insecure-Requests: 1
	# Sec-Fetch-Dest: document
	# Sec-Fetch-Mode: navigate
	# Sec-Fetch-Site: cross-site
	# Sec-GPC: 1
	# Cache-Control: max-age=0
	# TE: trailers
	var headers: PoolStringArray = PoolStringArray([
		"User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:96.0) Gecko/20100101 Firefox/96.0",
		"Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
		"Accept-Language: en-US,en;q=0.5",
		"DNT: 1",
		"Upgrade-Insecure-Requests: 1",
		"Sec-Fetch-Dest: document",
		"Sec-Fetch-Mode: navigate",
		"Sec-Fetch-Site: cross-site",
		"Sec-GPC: 1",
		"Cache-Control: max-age=0"
	])
	print("Downloading...")
	BrowserState.last_page = uri_input.text
	http.request(uri_input.text, headers, true, HTTPClient.METHOD_GET)

# http->request_completed
func _response(result: int, response_code: int, headers: PoolStringArray, body: PoolByteArray) -> void:
	if result == OK:
		print("Loading...")
		yield(get_tree(), "idle_frame")
		var response_body: String = body.get_string_from_utf8()
		html.load(response_body)
		print("Done!")
	else:
		print("ERROR: ", result, ", code: ", response_code)
