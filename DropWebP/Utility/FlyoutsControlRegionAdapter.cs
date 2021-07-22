using MahApps.Metro.Controls;
using Prism.Regions;
using System.Collections.Specialized;
using System.Windows;

namespace DropWebP.Utility
{
    class FlyoutsControlRegionAdapter : RegionAdapterBase<FlyoutsControl>
    {
        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="factory"></param>
        public FlyoutsControlRegionAdapter(IRegionBehaviorFactory factory)
            : base(factory)
        {
        }

        protected override void Adapt(IRegion region, FlyoutsControl regionTarget)
        {
            region.ActiveViews.CollectionChanged += (s, e) =>
            {
                if (e.Action != NotifyCollectionChangedAction.Add)
                {
                    return;
                }
                foreach (FrameworkElement element in e.NewItems)
                {
                    Flyout flyout = new Flyout();
                    flyout.Content = element;
                    flyout.DataContext = element.DataContext;
                    _ = regionTarget.Items.Add(flyout);
                }
            };
        }

        protected override IRegion CreateRegion()
        {
            return new AllActiveRegion();
        }
    }
}
