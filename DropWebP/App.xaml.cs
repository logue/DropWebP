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
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App : PrismApplication
    {
        private Mutex mutex = new(false, "DropWebP");

        /// <summary>
        /// 
        /// </summary>
        /// <returns></returns>
        protected override Window CreateShell()
        {
            return Container.Resolve<ShellWindow>();
        }

        /// <summary>
        /// コンテナを登録
        /// </summary>
        /// <param name="containerRegistry"></param>
        protected override void RegisterTypes(IContainerRegistry containerRegistry)
        {
            // ダイアログのデザインを揃える
            containerRegistry.RegisterDialogWindow<MetroDialogWindow>();
            // エンコーダー
            containerRegistry.RegisterSingleton<IWebPService, WebPService>();
        }

        /// <summary>
        /// 
        /// </summary>
        protected override void ConfigureRegionAdapterMappings(RegionAdapterMappings regionAdapterMappings)
        {
            regionAdapterMappings.RegisterMapping(typeof(FlyoutsControl), Container.Resolve<FlyoutsControlRegionAdapter>());
            base.ConfigureRegionAdapterMappings(regionAdapterMappings);
        }

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

        private void PrismApplication_Startup(object sender, StartupEventArgs e)
        {
            /*
            if (mutex != null)
            {
                mutex.ReleaseMutex();
                mutex.Close();
            }
            */
        }
    }
}
