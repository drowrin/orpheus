function Link(elem)
  if string.sub(elem.target, 1, 1) == "/" then
    elem.attributes["preload"] = "mouseover"
    elem.attributes["preload-images"] = "true"
  end
  return elem
end


function Image(elem)
  if string.sub(elem.src, 1, 2) == ".." then
    elem.src = string.sub(elem.src, 3)
  end

  if string.sub(elem.src, 1, 1) == "." then
    elem.src = string.sub(elem.src, 2)
  end

  return elem
end


function Header(elem)
  attr = {class="header-link"}
  link = pandoc.Link("#", "#" .. elem.attr.identifier, nil, attr)
  elem.content = elem.content .. {link}
  return elem
end
