function start()
    print("starting lua")
end

function run("Transform")
    
end


rust_log("LUA: executing rust_log function")
print(type(Transform.ref))
Transform:move_up(0.2)
