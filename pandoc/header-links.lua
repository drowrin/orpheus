function Header(elem)
    attr = {
        class = "header-link"
    }
    link = pandoc.Link("#", "#" .. elem.attr.identifier, nil, attr)
    elem.content = elem.content .. {link}
    return elem
end
