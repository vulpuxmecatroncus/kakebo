import {useState} from "react";

import {Box, Fab, Stack} from "@mui/material";

import LocalGroceryStoreIcon from '@mui/icons-material/LocalGroceryStore';
import StoreIcon from '@mui/icons-material/Store';
import RestaurantIcon from '@mui/icons-material/Restaurant';
import AddIcon from '@mui/icons-material/Add';

const QuickPurchaseMenu = () => {
    const [longPressed, setLongPressed] = useState(false);
    return (
        <Stack>
            <Box sx={{'& > :not(style)': {m: 2}}}>
                <Fab color="primary" aria-label="add" size="medium">
                    <RestaurantIcon/>
                </Fab>
                <Fab
                    color="primary"
                    aria-label="edit"
                    size="large"
                    onPointerDown={e => {
                        if (e.pointerType === 'touch' || e.pointerType === 'mouse') {
                            const timer = setTimeout(() => setLongPressed(true), 500);
                            e.target.addEventListener('pointerup', () => {
                                clearTimeout(timer);
                                setLongPressed(false);
                            }, {once: true});
                        }
                    }}
                >
                    {longPressed ? <StoreIcon/> : <AddIcon/>}
                </Fab>
                <Fab color="primary" aria-label="edit" size="medium">
                    <LocalGroceryStoreIcon/>
                </Fab>
            </Box>
        </Stack>
    );
};

export default QuickPurchaseMenu;