import Swiper from '/scripts/public/swiper-bundle.esm.browser.min.js';
export function swiperInit() {
    return new Swiper ('.swiper', {
        direction: 'horizontal',
        autoplay: true,
        pagination: {
            el: '.swiper-pagination'
        },
        navigation: {
            nextEl: '.swiper-button-next',
            prevEl: '.swiper-button-prev'
        }
    });
}