extends Button

func _ready():
	pressed.connect(_button_pressed)

func _button_pressed():
	get_node("/root/Main/Store").counter.decrement_count(1)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
