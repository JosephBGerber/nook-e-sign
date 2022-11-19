# Nook-E-Sign

Turn your nook into a remotely updatable e-paper display.

## Installation

This section will describe the process to install the nook-e-sign application on your Nooks.

#### 1. Create an installer.

The first step is to create an installer. An 8 GB micro sdcard is required. Download the latest version of NookSign.img
from the releases page. This file contains the installer for nook-e-sign. The installer will need to be written to the
sdcard.

One option for creating an installer is to use the balenaEtcher image writer application.

* Download this application from [balena.io](http://balena.io/etcher/).
* Run balenaEtcher.
* Select NookSign.img.
* Select the target sdcard.
* Finally, click "Burn" to create the installer. This process will overwrite the contents of the sdcard.

#### 2. Entering the Nook Manager.

Insert the installer sdcard into the sdcard port on the Nook. Hold the power button on the back of the Nook until the
"Turn Off NOOK" prompt opens. Click "Power off". Once the Nook has powered off, hold the power button again to restart
the Nook. Instead of booting into the Nook software, the Nook will boot into the "NookManager" application. If the Nook
boots into the Nook software, then the installer sdcard may not have been created correctly.

Once the "NookManager" is finished loading, you will be asked if you would like to "Enable Wireless?". Select no. You
will navigate to the "Main Menu".

#### 3. Backup the Nook.

Backing up your Nook is important if you want to revert these changes or recover a broken Nook. If you don't think this 
is needed then skip to the installing section.

Enter the "Main Menu". Click "Rescue". You will navigate to the "Rescue" menu. Click "Backup". You will
navigate to the "Create Backup" menu. Creating a backup will format the remaining space on the installer sdcard. Select
"Format remaining space on SD card". Once the sdcard is formatted, you can create a backup. Select "Create backup". This
will begin the backup process. This process can take up to 20 minutes. It is important that this process continues
uninterrupted. If the Nook dies, or the sdcard is removed during this process, then this step will need to be restarted.

Once a backup has been created, we can finally install the nook-e-sign application. Click "Back" twice to navigate to
the main menu.

#### 4. Installing Nook-E-Sign.

Now the Nook-E-Sign application can be installed. On the main menu, click "Install Nook-E-Sign". You will navigate to
the "Root your Nook" menu. Click "Root my device" to install the app. This will start the installation process. Once
this process is complete, the "Back" option will appear. Click "Back" and "Exit" to exit the "NookManager". Remove the
sdcard from the Nook. The Nook will automatically restart.

#### 5. Using Nook-E-Sign.

The Nook-E-Sign application coexists with the original Nook software. To access the Nook-E-Sign application, access the
Nook settings page. When clicked, a prompt will open allowing you to choose between the "Settings" app and the
Nook-E-Sign application. Within the Nook-E-Sign application, you can enter the hostname and name of your library to
automatically associate this device with your library.

## Credits

Credits to doozan for creating the NookManager application which was used to create the Nook-E-Sign installer. This
application could not exist without their work and the work of other before
them. [NookManager](https://github.com/doozan/NookManager)
