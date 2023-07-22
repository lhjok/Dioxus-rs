// import Swiper from 'swiper/bundle';
import Swiper from '/node_modules/swiper/swiper-bundle.min.mjs';

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
