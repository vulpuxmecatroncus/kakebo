import {Dispatch, SetStateAction, useState} from 'react';

import Home from "@mui/icons-material/Home";
import {Box, IconButton, Stack, Typography} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import {useTranslation} from 'react-i18next';
import {ChangeLanguage} from "./ChangeLanguage.tsx";
import UserConfigs from "./UserConfigs.tsx";

const Header = ({setDrawerOpen}: { setDrawerOpen: Dispatch<SetStateAction<boolean>> }) => {

    const [openChangeDialogue, setOpenChangeDialogue] = useState(false);

    return (
        <Stack component="header" boxShadow={1} direction={"row"} alignItems="center" justifyContent="space-between"
               p={2} bgcolor={"background.paper"}>
            <Box display="flex" alignItems="center" gap={2}>
                <IconButton edge="start" color="inherit" aria-label="menu" onClick={() => setDrawerOpen(true)}>
                    <MenuIcon/>
                </IconButton>
                <IconButton edge="start" color="inherit" aria-label="home">
                    <Home/>
                </IconButton>
            </Box>
            <Box display="flex" alignItems="center" gap={2}>
                <Typography variant="h5" sx={{textAlign: "center"}}>
                    {useTranslation().t('title')}
                </Typography>
                {/*Added to fix UserConfig empty space*/}
                <Box component="span" sx={{width: 56, display: "inline-block"}}/>
            </Box>
            <UserConfigs openChangeLanguage={setOpenChangeDialogue}/>
            <ChangeLanguage openChangeLanguageDialogue={openChangeDialogue}
                            setOpenChangeLanguageDialogue={setOpenChangeDialogue}/>
        </Stack>
    );
};

export default Header;