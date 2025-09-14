import {createTheme} from "@mui/material/styles";

function getCssVariable(name: string) {
    return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

export const theme = createTheme({
    palette: {
        primary: {main: getCssVariable('--primary-color') || '#1976d2'},
        secondary: {main: getCssVariable('--secondary-color') || '#dc004e'},
        background: {
            paper: getCssVariable('--background-paper') || '#f5f5f5',
            default: getCssVariable('--background-default') || '#7b6ccb',
        },
    },
    typography: {
        fontFamily: getCssVariable('--font-family') || 'Roboto, Arial, sans-serif',
    },
});