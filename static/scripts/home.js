import Swiper from './swiper-esm.js';
const swiper = new Swiper ('.swiper', {
    direction: 'horizontal',
    autoplay: true,
    pagination: {
        el: '.swiper-pagination'
    },
    navigation: {
        nextEl: '.swiper-button-next',
        prevEl: '.swiper-button-prev'
    }
})
