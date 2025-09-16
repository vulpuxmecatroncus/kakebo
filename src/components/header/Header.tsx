import React, {Dispatch, SetStateAction} from 'react';

import Home from "@mui/icons-material/Home";
import {Box, IconButton, Typography} from "@mui/material";
//SpeedDialIcon,
import MenuIcon from "@mui/icons-material/Menu";
import {useTranslation} from 'react-i18next';
import {ChangeLanguage} from "./ChangeLanguage.tsx";
import UserConfigs from "./UserConfigs.tsx";

const Header = ({setDrawerOpen}: { setDrawerOpen: Dispatch<SetStateAction<boolean>> }) => {

    const [openChangeDialogue, setOpenChangeDialogue] = React.useState(false);

    return (
        <Box component="header" boxShadow={1}>
            <Box display="flex" alignItems="center" justifyContent="space-between" p={2}>
                <IconButton edge="start" color="inherit" aria-label="menu" onClick={() => setDrawerOpen(true)}>
                    <MenuIcon/>
                </IconButton>
                <IconButton edge="start" color="inherit" aria-label="home">
                    <Home/>
                </IconButton>
                <Typography variant="h5" sx={{flexGrow: 1, textAlign: "center"}}>
                    {useTranslation().t('title')}
                </Typography>
                <UserConfigs openChangeLanguage={setOpenChangeDialogue}/>
            </Box>
            <ChangeLanguage openChangeLanguageDialogue={openChangeDialogue}
                            setOpenChangeLanguageDialogue={setOpenChangeDialogue}/>
        </Box>
    );
};

export default Header;