function Image(elem)
    if string.sub(elem.src, 1, 2) == ".." then
        elem.src = string.sub(elem.src, 3)
    end

    if string.sub(elem.src, 1, 1) == "." then
        elem.src = string.sub(elem.src, 2)
    end

    return elem
end
