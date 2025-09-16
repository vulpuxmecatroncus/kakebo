import {Dispatch, SetStateAction, useState} from 'react';

import Home from "@mui/icons-material/Home";
import {Box, IconButton, Typography} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import {useTranslation} from 'react-i18next';
import {ChangeLanguage} from "./ChangeLanguage.tsx";
import UserConfigs from "./UserConfigs.tsx";

const Header = ({setDrawerOpen}: { setDrawerOpen: Dispatch<SetStateAction<boolean>> }) => {

    const [openChangeDialogue, setOpenChangeDialogue] = useState(false);

    return (
        <Box component="header" boxShadow={1}>
            <Box display="flex" alignItems="center" justifyContent="space-between" p={2} pl={4} pr={4}>
                <Box display="flex" alignItems="center" gap={2}>
                    <IconButton edge="start" color="inherit" aria-label="menu" onClick={() => setDrawerOpen(true)}>
                        <MenuIcon/>
                    </IconButton>
                    <IconButton edge="start" color="inherit" aria-label="home">
                        <Home/>
                    </IconButton>
                </Box>
                <Typography variant="h5" sx={{textAlign: "center"}}>
                    {useTranslation().t('title')}
                </Typography>
                <Box display="flex" alignItems="center" gap={2}>
                    <UserConfigs openChangeLanguage={setOpenChangeDialogue}/>
                </Box>
            </Box>
            <ChangeLanguage openChangeLanguageDialogue={openChangeDialogue}
                            setOpenChangeLanguageDialogue={setOpenChangeDialogue}/>
        </Box>
    );
};

export default Header;