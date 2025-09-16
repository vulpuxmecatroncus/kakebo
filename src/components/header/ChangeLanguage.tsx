import React from 'react';
import Dialog from '@mui/material/Dialog';
import DialogTitle from '@mui/material/DialogTitle';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import {useTranslation} from 'react-i18next';

const languages = [
    {code: 'en', label: 'English'},
    //{code: 'ja', label: '日本語'},
    {code: 'es', label: 'Español'},
];

export const ChangeLanguage: React.FC<{
    openChangeLanguageDialogue: boolean;
    setOpenChangeLanguageDialogue: React.Dispatch<React.SetStateAction<boolean>>
}> = ({openChangeLanguageDialogue, setOpenChangeLanguageDialogue}) => {

    const {i18n} = useTranslation();
    //const [open, setOpen] = React.useState(false);
    const [selectedValue, setSelectedValue] = React.useState(i18n.language);

    const handleClose = (value?: string) => {
        setOpenChangeLanguageDialogue(false);
        if (value) {
            setSelectedValue(value);
            i18n.changeLanguage(value).then((_) => {
            });
        }
    };

    return (
        <Dialog onClose={() => handleClose()} open={openChangeLanguageDialogue}>
            <DialogTitle>Select Language</DialogTitle>
            <List>
                {languages.map((lang) => (
                    <ListItem key={lang.code} disableGutters>
                        <ListItemButton
                            selected={selectedValue === lang.code}
                            onClick={() => handleClose(lang.code)}
                        >
                            <ListItemText primary={lang.label}/>
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
        </Dialog>

    );
};