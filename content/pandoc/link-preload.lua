function Link(elem)
    if string.sub(elem.target, 1, 1) == "/" then
        elem.attributes["preload"] = "mouseover"
        elem.attributes["preload-images"] = "true"
    end
    return elem
end

