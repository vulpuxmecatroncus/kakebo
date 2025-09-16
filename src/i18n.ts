// src/i18n.ts
import i18n from "i18next";
import {initReactI18next} from "react-i18next";

const translations = {
    en: {translation: {
        title: "Welcome to Kakebo",
        months: [
            "January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"
        ]
    }},
    es: {translation: {
        title: "Bienvenido a Kakebo",
        months: [
            "Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio",
            "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"
        ]
    }},
}

await i18n.use(initReactI18next).init({
    resources: translations,
    lng: "en",
    fallbackLng: "en",
    interpolation: {escapeValue: false},
});