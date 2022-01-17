extends Node

var last_page: String

func _init() -> void:
	var file = File.new()
	if file.file_exists("user://save.data"):
		file.open("user://save.data", File.READ)
		var data = parse_json(file.get_as_text())
		file.close()
		last_page = data["last_page"]
	else:
		last_page = "https://hugo4it.com/"

func save() -> void:
	var file = File.new()
	file.open("user://save.data", File.WRITE)
	file.store_string(to_json({
		"last_page": last_page
	}))
	file.close()
