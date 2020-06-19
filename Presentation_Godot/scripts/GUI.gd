extends Node
class_name GUI
# DESCRIPTION:
# This node is parent of all GUI related things.
# It displays all the labels and shit and sends player input back to rustbridge

var rustbridge;
var selected_units = [];
var params;

func _ready():
	rustbridge = self.get_parent()
	params = self.get_node("/root/PresentationParams")


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func _unhandled_input(event):
	if event.is_action_pressed("presentation_spawn_unit"):
		var xy = rustbridge.get_viewport().get_mouse_position()/params.scale
		rustbridge.tmp_spawn_obj(xy)
	if event.is_action_pressed("tmp_receive_messages"):
		pass
	if event.is_action_pressed("deselect_all"):
		#print("deselect event registered")
		#self.get_tree().set_input_as_handled()
		pass
	if event.is_action_pressed("right_click"):
		for unit in selected_units:
			print("orer move for ", unit.get_name())
			var xy = rustbridge.get_viewport().get_mouse_position()/params.scale
			rustbridge.send_msg_move(unit.unique_id, xy)

func select_unit(node: Unit):
	if !selected_units.has(node):
		selected_units.push_back(node)
		print("unit", node ,"selected")


