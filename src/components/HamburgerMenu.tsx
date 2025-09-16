import {
    Divider,
    Drawer,
    List, ListItem,
    ListItemButton, ListItemIcon, ListItemText,
} from "@mui/material";
import InboxIcon from "@mui/icons-material/MoveToInbox";
import MailIcon from "@mui/icons-material/Mail";
import {Dispatch, FC, SetStateAction} from "react";

const HamburgerMenu: FC<{
    drawerOpen: boolean;
    setDrawerOpen: Dispatch<SetStateAction<boolean>>;
}> = ({ drawerOpen, setDrawerOpen }) => {
    return (
        <Drawer anchor="left" open={drawerOpen} onClose={() => setDrawerOpen(false)}>
            <List>
                {['Inbox', 'Starred', 'Send email', 'Drafts'].map((text, index) => (
                    <ListItem key={text} disablePadding>
                        <ListItemButton>
                            <ListItemIcon>
                                {index % 2 === 0 ? <InboxIcon/> : <MailIcon/>}
                            </ListItemIcon>
                            <ListItemText primary={text}/>
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
            <Divider/>
            <List>
                {['All mail', 'Trash', 'Spam'].map((text, index) => (
                    <ListItem key={text} disablePadding>
                        <ListItemButton>
                            <ListItemIcon>
                                {index % 2 === 0 ? <InboxIcon/> : <MailIcon/>}
                            </ListItemIcon>
                            <ListItemText primary={text}/>
                        </ListItemButton>
                    </ListItem>
                ))}
            </List>
        </Drawer>
    );
};

export default HamburgerMenu;