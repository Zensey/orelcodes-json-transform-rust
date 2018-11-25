
math.randomseed(os.time())
math.random(); math.random(); math.random()

request = function()
    wrk.method = "POST"
    wrk.headers["Content-Type"] = "application/json"
    path = "/"
    wrk.body   = '{"id": "' .. tostring(math.random()) .. '", "first_name": "Rand Name ' .. tostring(math.random()) .. '", "last_name": "Rand Last Name ' .. tostring(math.random()) .. '"}'
    return wrk.format("POST", path)

end