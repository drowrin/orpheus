const enabledQuery = window.matchMedia('(hover: hover) and (pointer: fine) and (min-width: 768px)')

const popover = document.querySelector('#image-lightbox')
const popoverImg = document.querySelector('#image-lightbox-img')

if (popover instanceof HTMLElement && popoverImg instanceof HTMLImageElement) {
  const images = document.querySelectorAll('post-card img')

  function enableImage(img) {
    if (!(img instanceof HTMLImageElement)) {
      return
    }

    if (img.closest('a')) {
      return
    }

    img.dataset.lightboxable = 'true'
    img.tabIndex = 0
    img.role = 'button'
    img.ariaLabel = img.alt ? `View larger image: ${img.alt}` : 'View larger image'

    function open() {
      if (!enabledQuery.matches) {
        return
      }

      popoverImg.src = img.currentSrc || img.src
      popoverImg.alt = img.alt || ''

      if ('showPopover' in popover) {
        popover.showPopover()
      }
    }

    img.addEventListener('click', open)

    img.addEventListener('keydown', (event) => {
      if (event.key === 'Enter' || event.key === ' ') {
        event.preventDefault()
        open()
      }
    })
  }

  images.forEach(enableImage)
}
