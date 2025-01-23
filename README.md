# xfce4 whiskermenu & panel customizer?

## Key Features

1. **XFCE4 Panel customization**: Allows you to change color and transparency of XFCE4 Panel.
2. **Whisker Menu customization**: Allows you to fully change color and transparency of Whisker menu on Mint themes.
3. **Whisker Menu Search customization**: Allows you to change color and transparency of Whisker search bar.
4. **All in one config**: Change everything in only one simple configuration file.

## Usage

1. **Backup current configurations**:
```
mkdir -p backup && \
cp /usr/share/themes/Mint-L-Dark/gtk-3.0/gtk-dark.css backup/ && \
cp ~/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml backup/ && \
cp ~/.config/xfce4/panel/*.rc backup/
```

2. **Navigate to the executable, create configuration file and edit it using a text editor**:
```
xfce4-transparent-whiskermenu --createconfig
```

3. **Close the xfconfdl using kill or killall**:
```
killall xfconfd
```

3. **View all avalable commands**:
```
xfce4-transparent-whiskermenu --help
```

4. **Run one of the update command to apply the configuration**:
```
sudo xfce4-transparent-whiskermenu --updateall
```

5. **Restart the panel or reboot to view changes**:
```
xfce4-panel -r
```

## Example

**Default config.toml file**
```
# default_paths
theme_path = '/usr/share/themes/Mint-L-Dark/gtk-3.0/gtk-dark.css'
whisker_menu_path = '~/.config/xfce4/panel/'
panel_path = '/home/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml'

# colors
base_color = "#000000"
opacity = 0.0
search_color = "#000000"
search_opacity = 0.0
```

<div align="center">
  <img src="https://github.com/Serters/xfce4-transparent-whiskermenu/blob/main/resources/example.png" alt="Example" width="100%">
</div>
