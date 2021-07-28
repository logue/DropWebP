using DropWebP.Interfaces;
using DropWebP.Service;
using DropWebP.Utility;
using DropWebP.Views;
using MahApps.Metro.Controls;
using Prism.DryIoc;
using Prism.Ioc;
using Prism.Regions;
using System.Threading;
using System.Windows;

namespace DropWebP
{
    /// <summary>
    /// Interaction logic for App.xaml.
    /// </summary>
    public partial class App : PrismApplication
    {
        /// <summary>
        /// Defines the mutex.
        /// </summary>
        private Mutex mutex = new(false, "DropWebP");

        /// <summary>
        /// The CreateShell.
        /// </summary>
        /// <returns>.</returns>
        protected override Window CreateShell()
        {
            return Container.Resolve<ShellWindow>();
        }

        /// <summary>
        /// コンテナを登録.
        /// </summary>
        /// <param name="containerRegistry">.</param>
        protected override void RegisterTypes(IContainerRegistry containerRegistry)
        {
            // ダイアログのデザインを揃える
            containerRegistry.RegisterDialogWindow<MetroDialogWindow>();
            // エンコーダー
            containerRegistry.RegisterSingleton<IWebPService, WebPService>();
        }

        /// <summary>
        /// The ConfigureRegionAdapterMappings.
        /// </summary>
        /// <param name="regionAdapterMappings">The regionAdapterMappings<see cref="RegionAdapterMappings"/>.</param>
        protected override void ConfigureRegionAdapterMappings(RegionAdapterMappings regionAdapterMappings)
        {
            regionAdapterMappings.RegisterMapping(typeof(FlyoutsControl), Container.Resolve<FlyoutsControlRegionAdapter>());
            base.ConfigureRegionAdapterMappings(regionAdapterMappings);
        }

        /// <summary>
        /// The PrismApplication_Exit.
        /// </summary>
        /// <param name="sender">The sender<see cref="object"/>.</param>
        /// <param name="e">The e<see cref="ExitEventArgs"/>.</param>
        private void PrismApplication_Exit(object sender, ExitEventArgs e)
        {
            if (mutex.WaitOne(0, false))
            {
                return;
            }

            MessageBox.Show("多重起動はできません。", "情報", MessageBoxButton.OK, MessageBoxImage.Information);
            mutex.Close();
            mutex = null;
            Shutdown();
        }

        /// <summary>
        /// The PrismApplication_Startup.
        /// </summary>
        /// <param name="sender">The sender<see cref="object"/>.</param>
        /// <param name="e">The e<see cref="StartupEventArgs"/>.</param>
        private void PrismApplication_Startup(object sender, StartupEventArgs e)
        {
        }
    }
}
